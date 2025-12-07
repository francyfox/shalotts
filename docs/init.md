# Genoplate

Easy create you own boilerplate from custom packages. Select packages in
WebUI and create generation seed for fast installing from CLI. Main idea 
from BatiJS.

This application for developers, who wants get rid of git templates. One 
boilerplate maker for all.

For example: for creating new step-by-step Elysia application we need to create:

- server (elysiajs)
- env typecheck and parser
- graceful functions
- cluster mode for production
- databases (drizzle orm/prizma)
- auth (authjs/lucia)
- analytics (posthog/sentry)
- logger
- etc
- directories
- -

**directories:**

- core
- - __test__
- - modules
- - - db
- - - analytics
- - - auth (fn to create provider)
- app
- bunfig.toml
- - __test__
- - modules
- - - auth (create instance with options)
- - server.ts
- - index.ts

Genoplate creator need to write templates, which will be implements to final 
build. Imports, plugin installations, snippets. All code need to separate 
and locked to version. Every packages and libs has connections. We cant use 
drizzle orm with prizma. And in some versions our snippets can be deprecated.

After all steps, when user selected all libs and packages, he can use 
special seed for saving his configuration. In future he can create 
boileplate from command 

```shell
genoplate create --seed=a44g5uy64kgk20slL # hash like
```