import { Entry } from "../../pages/api/feeds";

export const FeedContent = ({ data }: { data: Entry }) => {
  return (
    <div className="feed-content-wrapper">
      <a href={data.url}>{data.title}</a>
      <label>
        {new Date(data.created_date.replace(/-/g, "/")).toLocaleDateString()}
      </label>
    </div>
  );
};
