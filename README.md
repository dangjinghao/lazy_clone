# lazy_clone

While `wget` can download entire websites and their static CDN resources, it cannot capture dynamically loaded resources fetched via JavaScript. `lazy_clone` solves this by downloading static files, caching them, and streaming them to the client on demand, effectively cloning websites.
