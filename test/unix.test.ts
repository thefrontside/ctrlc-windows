import expect from "expect";

if (process.platform !== "win32") {
  describe("unix", () => {
    it("successfully requires", () => {
      require("../lib/index");
    });

    it("raises an error if you try to actually call the ctrlc() function", () => {
      expect.assertions(2);
      try {
        require("../lib/index").ctrlc(process.pid);
        throw new Error("invoking ctrlc() should definitely fail on windows");
      } catch (e) {
        if (e instanceof Error) {
          expect(e.name).toEqual("PlatformError");
          expect(e.message).toContain("non-windows");
        }
      }
      expect;
    });
  });
}
