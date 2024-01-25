window.addEventListener('load', () => {
  document.body.addEventListener('htmx:configRequest', function (evt) {
    const v = document.head.getAttribute('data-csrf');
    evt.detail.headers['X-LVSERVER-REQ'] = v;
  });
});
