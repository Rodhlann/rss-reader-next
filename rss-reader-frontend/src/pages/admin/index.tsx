import React, { useEffect, useState } from "react";
import { signIn, signOut, useSession } from "next-auth/react";
import RawFeed, { RawFeedItem } from "@/components/rawFeed";
import './Admin.css'

export default function Admin() {
  const { data: session } = useSession()

  const [ rawFeeds, setRawFeeds ] = useState<RawFeedItem[]>([]);
  const [ loading, setLoading ] = useState(true);
  const [ adding, setAdding ] = useState(false);
  
  const addFeed = async (feed: RawFeedItem) => {
    const response = await fetch('/api/admin/addFeed', {
      method: 'POST', 
      body: JSON.stringify({
        name: feed.name,
        url: feed.url,
        category: feed.category
      })
    })

    setAdding(false)

    if (!response.ok) {
      const json = await response.json()
      console.log('Error creating new Feed:', json)
    } else {
      const json = await response.json()
      setRawFeeds([...rawFeeds, json])
    }
  }

  const deleteFeed = async(id?: number) => {
    if (!id) return

    const response = await fetch('/api/admin/deleteFeed', {
      method: 'POST',
      body: JSON.stringify({ id })
    })

    if (!response.ok) {
      const json = await response.json()
      console.log('Error deleting Feed:', json)
    } else {
      setRawFeeds(rawFeeds.filter((feed) => feed.id !== id))
    }
  }

  useEffect(() => {
    if (session) {
      const fetchRawFeeds = async () => {
        try {
          const response = await fetch('/api/admin/rawFeeds')

          if (!response.ok) {
            throw new Error("Failed to fetch feed data")
          }

          const data = await response.json()
          setRawFeeds(data)
        } catch (error) {
          console.error("Error fetching feed data:", error)
        } finally {
          setLoading(false)
        }
      }

      fetchRawFeeds()
    }
  }, [session])

  if (!session) {
    return (
      <>
        Not signed in <br />
        <button onClick={() => window.location.href = "/"}>Home</button>
        <button onClick={() => signIn()}>Sign in</button>
      </>
    )
  }

  if (loading) {
    return <div>Fetching feed data...</div>
  }

  return (
    <>
      Admin: {session?.user?.name} <br />
      <button onClick={() => window.location.href = "/"}>Home</button>
      <button onClick={() => signOut()}>Sign out</button>
      
      <div className="raw-feed-list">
        <div className="raw-feed">
          <span>Feed Name</span><span>Feed URL</span><span>Feed Category</span>
        </div>
        { rawFeeds.map((feed) => <RawFeed key={`raw-feed-${feed.id}`} rawFeed={feed} deleteFeed={feed.id ? deleteFeed : undefined} />) }
        { adding && <RawFeed key={`raw-feed-add`} editing={adding} setAdding={setAdding} addFeed={addFeed} /> }
        { adding || <button className="add-raw-feed-button" onClick={() => setAdding(true)}>Add Feed</button> }
      </div>
    </>
  );
}