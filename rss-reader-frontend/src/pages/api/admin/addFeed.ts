import { getToken } from 'next-auth/jwt'
import { NextApiRequest, NextApiResponse } from 'next/types';

export default async function addFeed(req: NextApiRequest, res: NextApiResponse) {
  const token = await getToken({ req })
  const accessToken = token?.accessToken;

  if (accessToken) {
    const response = await fetch('https://rss-reader-service.shuttleapp.rs/admin', { 
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${accessToken}`,
        'Content-Type': 'application/json'
      },
      body: req.body
    })

    if (response.ok) {
      const json = await response.json()
      res.status(200).send(json)
      return
    } else {
      const text = await response.text()
      res.status(response.status).send({ error: `Server response: ${text}` })
      return
    }
  }
  
  res.status(401).send({ error: 'Unauthorized' })
}