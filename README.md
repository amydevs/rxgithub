# rxgithub (RustyFixGithub)

Embed Code Snippets, Images, Gifs, Videos, Gists, & more on Discord, Slack, Telegram, Twitter, etc.

![image](https://github.com/amydevs/rxgithub/assets/50583248/b2e00606-b2aa-4fdf-886b-55fef44d95b0)

### Usage

You can either:
- Add `rx` before your `github.com` link to make it `rxgithub.com`, OR
- Intall the bookmarklet it by highlighting the contents of the above code block, and then dragging it to your bookmark toolbar. Clicking on the bookmarklet will copy the rxgithub enabled URL to your clipboard.  ([source](/bookmarklet.js))
```html
javascript:(function()%7Bconst%20githubUrl%20%3D%20%22https%3A%2F%2Fgithub.com%22%3B%0Aif%20(window.location.href.startsWith(githubUrl))%20%7B%0A%20%20%20%20const%20rxGithubUrl%20%3D%20%22https%3A%2F%2Frxgithub.com%22%20%2B%20window.location.href.substring(githubUrl.length)%3B%0A%20%20%20%20navigator.clipboard.writeText(rxGithubUrl)%3B%0A%7D%7D)()%3B
```

## Embed Gists

![image](https://github.com/amydevs/rxgithub/assets/50583248/770088ed-0729-4608-9396-4ced395e6ec2)

## Embed SVGs

![image](https://github.com/amydevs/rxgithub/assets/50583248/494e4f73-d770-4e8d-b69f-95a6dd904643)

## Embed Images

![image](https://github.com/amydevs/rxgithub/assets/50583248/77e01c35-dc92-4f8d-a579-50d45c1cfb90)

## Embed Videos

![image](https://github.com/amydevs/rxgithub/assets/50583248/cf38f9f6-3cf6-41c9-95d1-50ad123c12d2)
![image](https://github.com/amydevs/rxgithub/assets/50583248/cb1d1031-c1f7-4277-88a4-217f2317e6bf)

## How Does it Work?

When a request hits an `rxgithub.com` URL, the user-agent is matched against a list of well-known bot user-agents. If the request appears to be from a bot, an HTML webpage is shown with all the required `<meta>` tags for Open-Graph compatibility. Otherwise, browser users are redirected to the original GitHub URL.

A `HEAD` request is made to the associated `raw.githubusercontent.com` URL to determine the `Content-Type` and it shows the appropriate `<meta>` tags for the content.

If the content is either code or an SVG, the server generates an image on the fly to serve to the open-graph crawler.
