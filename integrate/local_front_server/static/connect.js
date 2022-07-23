let target_endpoint = "http://127.0.0.1:8080";

function load() {
  console.log("get");
  let req = new XMLHttpRequest();
  req.open("GET", target_endpoint + "/links");
  req.responseType = 'json';
  req.onload = function() {
    console.log(req.response);
    let t = document.getElementById("links");
    t.textContent = t.textContent + req.response ;
  }
  req.send();
}

function post(j) {
  console.log("post");
  let req = new XMLHttpRequest();
  req.open("POST", target_endpoint + "/links");
  req.send(j);
}