<script>
 import { onMount, createEventDispatcher } from "svelte";
 onMount(async () => {
  var book = ePub("https://s3.amazonaws.com/moby-dick/moby-dick.epub");
  var rendition = book.renderTo("viewer", {
   width: "100%",
   height: "100vh",
   stylesheet: "/epub.css",
  });

  var displayed = rendition.display();

  displayed.then(function (renderer) {
   // -- do stuff
  });

  // Navigation loaded
  book.loaded.navigation.then(function (toc) {
   // console.log(toc);
  });

  var next = document.getElementById("next");
  next.addEventListener(
   "click",
   function () {
    rendition.next();
   },
   false
  );

  var prev = document.getElementById("prev");
  prev.addEventListener(
   "click",
   function () {
    rendition.prev();
   },
   false
  );

  var keyListener = function (e) {
   // Left Key
   if ((e.keyCode || e.which) == 37) {
    rendition.prev();
   }

   // Right Key
   if ((e.keyCode || e.which) == 39) {
    rendition.next();
   }
  };

  rendition.on("keyup", keyListener);
  document.addEventListener("keyup", keyListener, false);
 });
</script>

<div id="viewer" class="spreads text-white" />
<div id="prev" class="arrow">‹</div>
<div id="next" class="arrow">›</div>
