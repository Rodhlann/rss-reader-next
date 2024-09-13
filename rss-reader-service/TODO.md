# Caching

- [x] Automatically delete cached records after X minutes (this will probably require a service level cron job)
- [x] Abstract caching logic outside of xml.rs

# Feeds

- [ ] Allow updates of feeds
- [x] Allow bulk importing of feeds as json (result of RAW feeds query)

# Categories

- [ ] Do we even need a separate categories table? The frontend can just collect existing categories from returned feeds...
  - I was initially anticipating using this to help provide a categories dropdown when adding new feeds, but that seems potentially unnecessary
- [ ] Drop unused categories ? Cascade delete unused categories on Feed delete / Feed category update ?
- [ ] Add query to fetch categories ?