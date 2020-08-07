interface MockedConsole {
  log: jest.SpiedFunction<typeof console.log>;
}

export const withMockedConsole = (): MockedConsole => {
  const mockedConsole = ({
    log: undefined,
  } as unknown) as MockedConsole;

  beforeAll(() => {
    mockedConsole.log = jest
      .spyOn(global.console, "log")
      .mockImplementation(() => {});
  });

  afterAll(() => {
    mockedConsole.log.mockRestore();
  });

  return mockedConsole;
};
