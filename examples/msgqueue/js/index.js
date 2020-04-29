import("../pkg/index.js").catch(console.error);

jsmx_subscribe("setInterval","1000",function(msg) {
   console.log("read message from javascript",msg);
});

setInterval(function(){
   jsmx_push("setInterval","1000","message from javascript");
}, 1000);
