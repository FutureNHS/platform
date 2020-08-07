import { sendEvent } from "./events";
import EventGridClient from "azure-eventgrid";
import { withEnvVars } from "./test-helpers/environment";
import { withMockedConsole } from "./test-helpers/console";

jest.mock("azure-eventgrid");

const mockedEventGridClient = EventGridClient as jest.Mocked<
  typeof EventGridClient
>;

describe(sendEvent, () => {
  describe("with env vars", () => {
    withEnvVars({
      EVENTGRID_TOPIC_ENDPOINT: "http://my-topic.azure.local/events",
      EVENTGRID_TOPIC_KEY: "mySecretKey",
    });

    test("sends event to event grid", async () => {
      await sendEvent({
        subject: "test",
        eventType: "test",
        data: {},
        dataVersion: "1",
      });

      expect(mockedEventGridClient).toHaveBeenCalledTimes(1);
      expect(
        mockedEventGridClient.prototype.publishEvents
      ).toHaveBeenCalledWith("my-topic.azure.local", [
        {
          id: expect.any(String),
          subject: "test",
          eventType: "test",
          eventTime: expect.any(Date),
          data: {},
          dataVersion: "1",
        },
      ]);
    });
  });

  describe("without env vars", () => {
    const mockedConsole = withMockedConsole();

    test("logs event to console", async () => {
      await sendEvent({
        subject: "test",
        eventType: "test",
        data: {},
        dataVersion: "1",
      });

      expect(mockedConsole.log).toHaveBeenCalledTimes(1);
    });
  });
});
