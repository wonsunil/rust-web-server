const $loginButton = document.querySelector("#login");
$loginButton.addEventListener("click", () => {
    fetch("/user/login", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            id: "test",
            password: "test"
        })
    })
    .then(req => req.json())
    .then(res => {
        console.log(res);

        console.log(res.status, document.cookie);

        if(res.status === "200" && document.cookie.length === 0 && document.cookie.split("\n")[0].length === 0) {
            document.cookie = res.cookie;
        };
    });
});