# `boteco`

`boteco` is a CLI to create redirections in CloudFlare to Zoom meetings.

## Requirements

### CloudFlare page rules

In the domain you want to use, you need to create two page rules, one for the public URL and one for the private URL. You can skip any of them if you don't want to use it.

`boteco` only **edits** these rules, it **does not create** new rules, so you can have the same URL for different meetings over time.

#### Public URL page rule

URL that will redirect to the Zoom meeting without the meeting password embedded in it. For example, if you set the macth rule to `public.house/` this will redirect to something like `https://zoom.us/j/12345678`. Users will need to know and manuall type de password to join.

#### Private URL page rule

URL that will redirect to the Zoom meeting with the meeting password embedded in it. For example, if you set the macth rule to `public.house/only-closest-friends-will-get-this-one` this will be redirect to something like `https://us02web.zoom.us/j/12345678?pwd=abcdef`.

#### Example

![Example of a page rule on CloudFlare](pagerule.png)

### Environment variables

| Variable | Description |
|---|---|
`BOTECO_CLOUD_FLARE_API_TOKEN` | API token to access CloudFlare |
`BOTECO_CLOUD_FLARE_ZONE_ID` | Zone ID of the CloudFlare domain |
`BOTECO_PUBLIC_URL` | URL to be redirected to the Zoom meeting without the password  |
`BOTECO_PRIVATE_URL` | URL to be redirected to the Zoom meeting with the password |

## Usage

Assuming public and private URLs to be, respectively, `public.house/` and `public.house/only-closest-friends-will-get-this-one`:

```console
$ boteco "https://us02web.zoom.us/j/12345678?pwd=abcdef"
https://public.house/only-closest-friends-will-get-this-one => https://us02web.zoom.us/j/12345678?pwd=abcdef
https://public.house/ => https://zoom.us/j/12345678
```

### Development

Activate debug mode with `RUST_LOG=debug` environment variable.
