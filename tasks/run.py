"""
Run tasks.
"""
import sys
import threading
import webbrowser

from invoke import task

from tasks import BIN
from tasks.util import cargo


@task(default=True)
def bin(ctx):
    """Run the binary crate."""
    # Because we want to accept arbitrary arguments, we have to ferret them out
    # of sys.argv manually.
    cargo(ctx, 'run', *sys.argv[2:], crate=BIN, wait=False)


@task(help={
    'port': "Port to have the docs' HTTP server listen on",
    'reload': "Whether live reload of the docs should be enabled",
    'verbose': "Whether to display verbose logging output of the server",
})
def docs(ctx, port=8000, reload=False, verbose=False):
    """"Run" the docs.

    This starts an HTTP server for serving the docs (with optional live reload)
    and previews them in the default web browser.
    """
    addr = '127.0.0.1:%s' % port

    build_error_event = threading.Event()

    def open_browser(url):
        """Open browser with compiled docs.
        Runs in a separate thread to sleep for a while while the docs build.
        """
        # few seconds should be enough for a successful build
        build_error_event.wait(timeout=2.0)
        if not build_error_event.is_set():
            webbrowser.open_new_tab(url)

    opener_thread = threading.Thread(target=open_browser,
                                     args=('http://%s' % addr,))
    opener_thread.start()

    # run server which will take some time to execute the build;
    # if that fails, we signal the event to prevent the browser from opening
    server = ctx.run('mkdocs serve --dev-addr %s --%slivereload %s' % (
        addr,
        '' if reload else 'no-',
        '--verbose' if verbose else '',
    ), pty=True)
    if not server.ok:
        build_error_event.set()
        opener_thread.join()
