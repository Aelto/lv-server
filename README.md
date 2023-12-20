> view the [htmx.md](htmx.md) file to see the HTMX specific notes 

# Problem 1: htmx, UI routes, fragment routes, and avoiding url clashes
## problem
At first it might be simpler to organize the pages and their
endpoints using basic urls, for example:
- `GET user/profile` ~> render profile 
  - `GET user/profile/edit` ~> render profile edit form
    - `POST user/profile` ~> update profile & return updated profile

But then what happens if we now need `GET user/{slug}/something` ?

## solution
It can be interesting to split the UI routes & the fragments routes
- `GET user/{slug}/profile` ~> render profile, listen for events from below fragments
- `GET app/profile-edit` ~> render profile edit form
  - `POST app/profile-edit` ~> update profile & send `fetch-profile` event

The prefix `app` is reserved for all fragments so they can add as
many endpoints as they need.

# Problem 2: fragments, genericity, events
## problem
Now that fragments have their own prefix to separate them from the
pages, how about the genericity of the fragments.

It might be tempting to have the following fragments:
- `LibraryList` ~> lists the libraries 
- `LibraryForm` ~> form to create a library
  - `POST app/library-form`

But then when a library is created multiple solutions are possible
to update the UI:
- trigger a `libraries-changed` event for the list, which implies the list should listen for this event. Which is fine as long as there is only one event, but what happens when there are multiple fragments and the amount of events grows?
- the POST returns the HTML of the library, which is appended to the list, but the form & the list are now __tightly__ coupled. Which means they might as well be a single fragment together.

## solution
The problem comes from the fact the fragments are too specialized
to be considered generic fragments so they might as well be coupled.

Now whether they use events or append/replace DOM elements is up to preferences, but generally events encourage a more "stateless" way of thinking. Which can also allow for some of these fragments to be used outside their original places.