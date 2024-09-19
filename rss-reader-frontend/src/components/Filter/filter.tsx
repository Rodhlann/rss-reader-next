import { Dispatch, MouseEvent, SetStateAction } from "react"

type FilterInput = { 
  categories?: string[],
  categoryFilter?: string,
  setCategoryFilter: Dispatch<SetStateAction<string | undefined>>
}

export const Filter = ({ categories, categoryFilter, setCategoryFilter }: FilterInput) => {

  const handleClick = (event: MouseEvent<HTMLInputElement>, selection: string) => {
    if (selection === categoryFilter) {
      setCategoryFilter(undefined)
      // @ts-ignore
      event.target.checked = false;
    } else {
      setCategoryFilter(selection)
    }
  }

  return (
    <div className="radio-group">
      { 
        categories?.map((category, idx) => (
          <label key={`category-${category}-${idx}`} className={`radio-option ${category === categoryFilter ? 'radio-checked' : ''}`}>
            <input 
              type="radio" 
              name="options" 
              value={category} 
              onClick={(e) => handleClick(e, category)}
            />
            {category}
          </label>
        ))
      }
    </div>
  )
}