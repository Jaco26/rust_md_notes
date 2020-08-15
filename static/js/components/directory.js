import DirectoryItems from './directory-items.js'

export default {
  functional: true,
  name: 'Directory',
  props: {
    child_dirs: Array,
    files: Array,
    name: String,
  },
  render(h, ctx) {
    return h(
      'div',
      { class: 'directory' },
      [
        h(
          'div',
          {
            class: 'directory__name',
            on: { click: () => console.log(`You clicked this directory ${ctx.props.name}`) }
          },
          ctx.props.name
        ),
        h(
          DirectoryItems,
          { props: { child_dirs: ctx.props.child_dirs, files: ctx.props.files } },
        )
      ]
    )
  }
}

/*

  *** Recursive Directory Render Strategy ***

  ** Components **

  - Directory
    
    props:
    - name
    - files
    - child_dirs

    description:
    - renders a div with two children:
      - props.name
      - <DirectoryItems props.files props.child_dirs />



  - DirectoryItems

    props:
    - files
    - child_dirs

    description:
    - renders a <ul class="dir-items__files"> for file in props.files
    - renders a <ul class="dir-items__child-dirs"> for dir in props.child_dirs
      - for each dir, render an <li> with a <Directory /> child, passing this dir's: files, child_dirs, name


*/