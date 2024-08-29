import { getToken } from 'next-auth/jwt'
import { NextApiRequest, NextApiResponse } from 'next/types';

export default async function rawFeeds(req: NextApiRequest, res: NextApiResponse) {
  const token = await getToken({ req })
  const accessToken = token?.accessToken;

  if (accessToken) {
    const response = await fetch('https://rss-reader-service.shuttleapp.rs/admin', { 
      headers: {
        'Authorization': `Bearer ${accessToken}`
      }
    })

    if (response.ok) {
      const json = await response.json();
      res.send(json)
      return
    }
  }
  
  res.status(401).send({ error: 'Unauthorized' })
}