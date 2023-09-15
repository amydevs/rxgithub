const githubUrl = "https://github.com";
if (window.location.href.startsWith(githubUrl)) {
    const rxGithubUrl = "https://rxgithub.com" + window.location.href.substring(githubUrl.length);
    navigator.clipboard.writeText(rxGithubUrl);
}