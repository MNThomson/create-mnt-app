import express, { Request, Response, NextFunction } from "express";
import loginRoutes from "./routes/login";
import morgan from "morgan";

const app = express();
app.use(morgan("dev"));
app.use(express.json());

app.use((err: Error, req: Request, res: Response, next: NextFunction) => {
  console.error(err.stack);
  res.status(500).send("Something went wrong");
});

app.get("/api/user", (req: Request, res: Response) => {
  res.send("Hello world");
});

app.use("/api/user/login", loginRoutes);

app.listen(3000, "0.0.0.0", () => {
  console.log(`Server running at http://0.0.0.0:3000`);
});
