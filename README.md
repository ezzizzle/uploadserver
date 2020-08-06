# UploadServer

Simple webserver that allows uploading files to a specified directory.

# Usage

You need to set the following environment variables:

* `FILE_DIRECTORY` - Where to save the files
* `BASE_URL` - The url where files are served from
* `SERVER_PORT` - The port to listen on
* `UPLOAD_TOKEN` - A value that must be passed in the `token` header to allow uploading

```
FILE_DIRECTORY=/tmp/uploader BASE_URL=https://domain.com/files/ SERVER_PORT=5000 UPLOAD_TOKEN=1234567 uploadserver

# Or with Docker
docker run --rm -ti -p 5000:5000 -e FILE_DIRECTORY=/tmp/uploader -e SERVER_PORT=5000 -e BASE_URL=https://domain.com/files/ -e UPLOAD_TOKEN=1234567 -v /host/path:/tmp/uploader uploadserver:latest
```

# Making Requests

Upload a file using a multi-part form field with a field name of `file`. You also need to set two values in the HTTP header:

1. `token` - This must match the `UPLOAD_TOKEN` environment variable specified when starting the server
2. `uploadpath` - The relative path to upload the given file to
    * This prepends the value of the `FILE_DIRECTORY` environment variable set on the server
    * The `uploadpath` value must include the filename (e.g. `relative/path/to/file.txt`)
