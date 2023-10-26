let request = new XMLHttpRequest()
request.open("GET", "/content", false)
request.send()
let content = JSON.parse(request.responseText)
for (let image of content.images) {
    document.querySelector("body>div").innerHTML += `<div class="video">
    <a href="/videos/0">
        <img src="/images/${image}">
    </a>
    <div>
        <a href="/users/0">
            <img src="/icons/0">
        </a>
        <div>
            <a href="/videos/0">Sample Video</a>
            <p>Sample User</p>
        </div>
    </div>
    </div>`
}