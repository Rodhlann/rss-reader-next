import { useState } from "react"

export type RawFeedItem = {
  name: string,
  url: string,
  category: string,
  id?: number,
}

export type RawFeedInput = {
  rawFeed?: RawFeedItem,
  editing?: boolean
  setAdding?: React.Dispatch<React.SetStateAction<boolean>>
  addFeed?: (feed: RawFeedItem) => {}
  deleteFeed?: (id?: number) => {}
}

export default function RawFeed({ rawFeed, setAdding, addFeed, deleteFeed, editing = false }: RawFeedInput) {
  const [editMode, setEditMode] = useState(editing);
  const [name, setName] = useState<string>(rawFeed?.name || '');
  const [url, setUrl] = useState<string>(rawFeed?.url || '');
  const [category, setCategory] = useState<string>(rawFeed?.category || '');
  const [validationError, setValidationError] = useState('');
  
  const cancelEditMode = () => {
    setName(rawFeed?.name || '')
    setUrl(rawFeed?.url || '')
    setCategory(rawFeed?.category || '')
    setValidationError('')
    setAdding && setAdding(false)
    setEditMode(false)
  }

  const submitChanges = () => {
    if (!name || !url || !category) {
      const errs = []
      if (!name) errs.push('name')
      if (!url) errs.push('url')
      if (!category) errs.push('category')
      setValidationError(`Must provide: ${errs.join(', ')}`)
    } else {
      addFeed && addFeed({ name, url, category})
      setEditMode(false)
    }
  }

  if (editMode) {
    return (
      <>
        <div className="raw-feed">
          <input type='text' value={name} onChange={(e) => setName(e.target.value)} />
          <input type='text' value={url} onChange={(e) => setUrl(e.target.value)} />
          <input type='text' value={category} onChange={(e) => setCategory(e.target.value)} />
          <button onClick={cancelEditMode}>Cancel</button>
          <button onClick={submitChanges}>Submit</button>
        </div>
        { validationError && 
          <div>
            <label style={{color: 'red'}}>{ validationError }</label>
          </div>
        }
      </>
    )
  }

  return (
    <div className="raw-feed">
      <span>{name}</span>
      <a href={url}>{url}</a>
      <span>{category}</span>
      <button onClick={() => setEditMode(true)}>Edit</button>
      { deleteFeed && <button onClick={() => deleteFeed(rawFeed?.id)}>Delete</button> }
    </div>
  )
}