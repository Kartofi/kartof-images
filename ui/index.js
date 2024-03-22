async function load() {
  let form = document.getElementById("uploadForm");
  let inpFile = document.getElementById("inpFile");

  let url = document.getElementById("img-url");

  form.addEventListener("submit", async (event) => {
    event.preventDefault();
    if (inpFile.files.length == 0) {
      window.alert("Please select one image before clicking upload!");
      return;
    }
    if (inpFile.files[0].size > 20_000_000) {
      window.alert("File is too large max file size is 20MB!");
      return;
    }

    const formData = new FormData();

    formData.append("inpFile", inpFile.files[0]);

    let data = await fetch("/upload", {
      method: "POST",
      body: formData,
    }).catch(function (error) {
      window.alert("Error uploading file!");
      return;
    });

    let text = await data.text();
    form.reset();
    document.getElementById("file_upload").innerHTML =
      "Click to upload a file";
    url.innerHTML =
      "Your image url is <a href='" + text + "'>" + text + "</a> <br /> <br />";
    window.alert("Successfully uploaded image!");
  });
}

function getFile() {
  document.getElementById("inpFile").click();
}

function sub(obj) {
  var file = obj.value;
  var fileName = file.split("\\");
  document.getElementById("file_upload").innerHTML =
    fileName[fileName.length - 1];
  document.myForm.submit();
  event.preventDefault();
}
