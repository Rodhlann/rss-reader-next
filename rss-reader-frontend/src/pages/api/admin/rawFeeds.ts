import { getToken } from 'next-auth/jwt'
import { NextApiRequest, NextApiResponse } from 'next/types';

export default async function rawFeeds(req: NextApiRequest, res: NextApiResponse) {
  const token = await getToken({ req })
  const accessToken = token?.accessToken;

  if (accessToken) {
    const response = await fetch(`${process.env.RSS_READER_SERVICE_URL}/admin`, { 
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