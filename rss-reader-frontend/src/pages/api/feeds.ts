import { NextApiRequest, NextApiResponse } from "next";

type Entry = {
  title: string,
  url: string,
  created_date: string
}

export type Feed = {
  name: string,
  category: string,
  entries: Entry[],
}

export default async function feeds(req: NextApiRequest, res: NextApiResponse) {
    const response = await fetch('https://rss-reader-service.shuttleapp.rs/feeds')

    if (response.ok) {
      const json = await response.json();
      res.send(json)
      return
    }

    res.status(response.status).send(await response.text())
}