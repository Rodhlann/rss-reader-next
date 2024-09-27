import { NextApiRequest, NextApiResponse } from "next";

export type Entry = {
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
  const max_entries = req.query.max_entries
  const duration = req.query.duration
  const response = await fetch(`https://rss-reader-service.shuttleapp.rs/feeds?max_entries=${max_entries}&duration=${duration}`)

  if (response.ok) {
    const json = await response.json();
    res.send(json)
    return
  }

  res.status(response.status).send(await response.text())
}