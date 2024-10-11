import { getToken } from 'next-auth/jwt'
import { NextApiRequest, NextApiResponse } from 'next/types';

export default async function deleteFeed(req: NextApiRequest, res: NextApiResponse) {
  const token = await getToken({ req })
  const accessToken = token?.accessToken;

  const { id } = JSON.parse(req.body);

  if (accessToken) {
    const response = await fetch(`${process.env.RSS_READER_SERVICE_URL}/admin/${id}`, { 
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${accessToken}`,
      }
    })

    if (response.ok) {
      res.status(200).send(`Successfully deleted Feed`)
      return
    } else {
      const text = await response.text();
      res.status(response.status).send({ error: `Server response: ${text}` })
      return
    }
  }
  
  res.status(401).send({ error: 'Unauthorized' })
}