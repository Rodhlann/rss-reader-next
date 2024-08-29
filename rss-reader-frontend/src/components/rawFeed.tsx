import { useState } from "react"

export type RawFeedItem = {
  id: number,
  name: string,
  url: string,
  category: string
}

export type RawFeedInput = {
  rawFeed: RawFeedItem
}

export default function RawFeed({ rawFeed }: RawFeedInput) {
  const [editMode, setEditMode] = useState(false);
  const [name, setName] = useState(rawFeed.name);
  const [url, setUrl] = useState(rawFeed.url);
  const [category, setCategory] = useState(rawFeed.category);

  const cancelEditMode = () => {
    setName(rawFeed.name)
    setUrl(rawFeed.url)
    setCategory(rawFeed.category)
    setEditMode(false)
  }

  const submitChanges = () => {
    console.log('New name:', name)
    console.log('New url:', url)
    console.log('New category:', category)
    setEditMode(false)
  }

  if (editMode) {
    return (
      <>
        <div>
          <input type='text' value={name} onChange={(e) => setName(e.target.value)} />
          |
          <input type='text' value={url} onChange={(e) => setUrl(e.target.value)} />
          |
          <input type='text' value={category} onChange={(e) => setCategory(e.target.value)} />
          |
          <button onClick={cancelEditMode}>Cancel</button>
          |
          <button onClick={submitChanges}>Submit</button>
        </div>
      </>
    )
  }

  return (
    <div>
      {name} | {url} | {category} | <button onClick={() => setEditMode(true)}>Edit</button>
    </div>
  )
}