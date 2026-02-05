import { expect, test, describe } from "vitest";
import { faker } from "@faker-js/faker";
import {
  Group,
  Measurement,
  Page,
  plus100,
  Scale,
  TakeoffStateHandler,
  Unit,
} from "../index.js";

const generatePageIds = (count: number): string[] => {
  return Array.from({ length: count }, (_, i) => `page-${i}`);
};

const createManyScales = (
  count: number,
  options?: {
    id?: string;
    pageId?: string;
    scale?: {
      pixelDistance: number;
      realDistance: number;
      unit: Unit;
    };
  },
): Scale[] => {
  const {
    id,
    pageId = "1",
    scale = { pixelDistance: 1, realDistance: 1, unit: "Meters" },
  } = options || {};
  return Array.from({ length: count }, (_, i) => ({
    id: id || `${faker.database.mongodbObjectId()}-${i}`,
    type: "Default",
    pageId,
    scale: {
      pixelDistance: scale.pixelDistance,
      realDistance: scale.realDistance,
      unit: scale.unit,
    },
  }));
};

const createManyGroups = (count: number, options?: Partial<Group>): Group[] => {
  const {
    id = `group-${count}`,
    name,
    measurementType = "Area",
  } = options || {};
  return Array.from({ length: count }, (_, i) => ({
    id: `${id}-${i}`,
    name: name || `Group ${i}`,
    measurementType,
  }));
};

const generatePoints = (
  type: Measurement["type"],
): Pick<Measurement, "type" | "points"> => {
  switch (type) {
    case "Rectangle":
      const x1 = faker.number.float({ min: 0, max: 1000 });
      const y1 = faker.number.float({ min: 0, max: 1000 });
      return {
        type,
        points: [
          { x: x1, y: y1 },
          { x: x1 + 1, y: y1 + 1 },
        ],
      };
    case "Polygon":
      return {
        type,
        points: [
          { x: 0, y: 0 },
          { x: 1, y: 0 },
          { x: 1, y: 1 },
          { x: 0, y: 1 },
        ],
      };
    case "Polyline":
      return {
        type,
        points: [
          { x: 0, y: 0 },
          { x: 1, y: 1 },
          { x: 2, y: 2 },
        ],
      };
    case "Count":
      return {
        type,
        points: [{ x: 0, y: 0 }],
      };
  }
};

const createManyMeasurements = (
  count: number,
  options?: Partial<Omit<Measurement, "points">>,
): Measurement[] => {
  const {
    id,
    type = faker.helpers.arrayElement(["Rectangle", "Polygon"]),
    pageId = "1",
    groupId = "1",
  } = options || {};

  return Array.from({ length: count }, (_, i) => ({
    id: id || `${faker.database.mongodbObjectId()}-${i}`,

    pageId,
    groupId,
    ...(generatePoints(type) as any),
  }));
};

type UpsertHandler =
  | {
      type: "scale";
      value: Scale;
    }
  | {
      type: "group";
      value: Group;
    }
  | {
      type: "measurement";
      value: Measurement;
    }
  | {
      type: "page";
      value: Page;
    };

describe("TakeoffStateHandler", () => {
  test("should get measurements by group id", () => {
    const state = new TakeoffStateHandler({
      pages: [],
      groups: [],
      measurements: [],
      scales: [],
    });
    const measurements = state.getMeasurementsByGroupId("1");
    expect(measurements).toEqual([]);
  });

  test("should process measurements and groups", () => {
    const state = new TakeoffStateHandler({
      pages: [],
      groups: [],
      measurements: [],
      scales: [],
    });
    const pageIds = generatePageIds(10);
    const scales = pageIds.flatMap((pageId) => createManyScales(1, { pageId }));
    const groups = createManyGroups(10);
    const measurements = createManyMeasurements(100).map((measurement) => {
      const scale = faker.helpers.arrayElement(scales);

      const group = faker.helpers.arrayElement(groups);
      return {
        ...measurement,
        scaleId: scale.id,
        pageId: scale.pageId,
        groupId: group.id,
      };
    });

    for (const group of groups) {
      state.upsertGroup(group);
    }

    for (const scale of scales) {
      state.upsertScale(scale);
    }
    for (const measurement of measurements) {
      state.upsertMeasurement(measurement);
    }
    const testMeasure = state.getMeasurement(
      faker.helpers.arrayElement(measurements).id,
    );
    expect(testMeasure).toBeDefined();

    expect(testMeasure?.area).toBeDefined();
    expect(testMeasure?.length).toBeDefined();

    expect(testMeasure?.area?.display("Meters")).toBe("1 m²");
    const testGroup = state.getGroup(testMeasure!.measurement.groupId)!;
    expect(testGroup?.area).toBeDefined();

    const groupMeasurementCount = testGroup.count!;
    expect(testGroup.points).toBe(groupMeasurementCount * 4);
    expect(testGroup.length?.getConvertedValue("Meters")).toBe(
      groupMeasurementCount * 4,
    );
    expect(testGroup.area?.getConvertedValue("Meters")).toBe(
      groupMeasurementCount,
    );

    const newTestScale: Scale = {
      type: "Default",
      id: faker.database.mongodbObjectId(),
      pageId: faker.string.uuid(),
      scale: {
        pixelDistance: 10,
        realDistance: 0.5,
        unit: "Feet",
      },
    };
    const newTestMeasure: Measurement = {
      type: "Rectangle",
      id: faker.database.mongodbObjectId(),
      pageId: newTestScale.pageId,
      groupId: faker.helpers.arrayElement(groups).id,
      points: [
        { x: 0, y: 0 },
        { x: 10, y: 10 },
      ],
    };

    state.upsertMeasurement(newTestMeasure);
    const newMeasureWrapper = state.getMeasurement(newTestMeasure.id);
    console.log(newMeasureWrapper?.area, newMeasureWrapper?.length);
    expect(newMeasureWrapper?.area).toBeNull();
    expect(newMeasureWrapper?.length).toBeNull();
    expect(state.getMeasurementsMissingScale().length).toBe(1);
    state.upsertScale(newTestScale);
    expect(newMeasureWrapper?.area).toBeDefined();
    expect(newMeasureWrapper?.length).toBeDefined();
    expect(newMeasureWrapper?.area?.display("Feet")).toBe("0.25 ft²");
    expect(newMeasureWrapper?.length?.display("Feet")).toBe("2 ft");
  });

  test("should handle items in random order", () => {
    const state = new TakeoffStateHandler();
    const calls: UpsertHandler[] = [];
    const pageIds = generatePageIds(25);
    const scales = pageIds.flatMap((pageId) => createManyScales(1, { pageId }));
    const groups = createManyGroups(10);

    const measurements: Measurement[] = createManyMeasurements(1000).map(
      (measurement) => {
        const scale = faker.helpers.arrayElement(scales);
        const group = faker.helpers.arrayElement(groups);
        return {
          ...measurement,

          scaleId: scale.id,
          pageId: scale.pageId,
          groupId: group.id,
        };
      },
    );
    for (const scale of scales) {
      calls.push({
        type: "scale",
        value: scale,
      });
    }
    for (const group of groups) {
      calls.push({
        type: "group",
        value: group,
      });
    }
    for (const measurement of measurements) {
      calls.push({
        type: "measurement",
        value: measurement,
      });
    }
    for (const call of faker.helpers.shuffle(calls)) {
      switch (call.type) {
        case "scale":
          state.upsertScale(call.value);
          break;
        case "group":
          state.upsertGroup(call.value);
          break;
        case "measurement":
          state.upsertMeasurement(call.value);
          break;
      }
    }

    expect(state.getMeasurementsMissingScale().length).toBe(0);
    const sampledMeasurements = faker.helpers.arrayElements(measurements, 10);
    for (const measurement of sampledMeasurements) {
      const testMeasure = state.getMeasurement(measurement.id);
      expect(testMeasure).toBeDefined();
      expect(testMeasure?.area).toBeDefined();
      expect(testMeasure?.length).toBeDefined();
      expect(testMeasure?.area?.display("Meters")).toBe("1 m²");
      const testGroup = state.getGroup(testMeasure!.measurement.groupId)!;
      const groupMeasurements = state.getMeasurementsByGroupId(
        testMeasure!.groupId,
      );
      expect(groupMeasurements.length).toBe(testGroup.count);
      for (const measurement of groupMeasurements) {
        expect(measurement.area).toBeDefined();
        expect(measurement.length).toBeDefined();
        expect(measurement.area?.display("Meters")).toBe("1 m²");
        expect(measurement.length?.display("Meters")).toBe("4 m");
        expect(measurement.points).toBe(4);
      }
      expect(testGroup?.area).toBeDefined();
      expect(testGroup?.length).toBeDefined();
    }
  });
});
