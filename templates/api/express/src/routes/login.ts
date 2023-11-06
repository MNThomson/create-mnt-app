import { Router, Request, Response } from "express";
import { User } from "../models/user";

const router = Router();

router.get("/", (req: Request, res: Response) => {
  res.send("Hello world!");
});

router.post("/", (req: Request, res: Response) => {
  const user: User = {
    id: 1,
    name: req.body.name,
  };

  res.status(201).json(user);
});

export default router;
