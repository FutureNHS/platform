export const withEnvVars = (vars: { [name: string]: string }) => {
  beforeAll(() => {
    Object.assign(process.env, vars);
  });

  afterAll(() => {
    for (const key of Object.keys(vars)) {
      // Ensure the environment variable has not been modified
      expect(process.env[key]).toBe(vars[key]);
      delete process.env[key];
    }
  });
};
