# Daily
**Get your day together.**

## Ideas
* Terminal greeter <-
* Notes to self
* What's next
  * Configurable (GH issues, PRs, etc.)
* Optional device sync
* Task selector
* GUI?
  * Goes through process, e.g. for a Github issue:
    ```mermaid
    graph TD;
    Select[Select a Task] --> Editor[Open Code Editor]
    --> AskFinished{Ask if task is finished}

    AskFinished
    -- Yes --> Issue[Open issue comment editor]
    --> Close[Close issue]
    --> RmTodo[Remove from Todos]

    AskFinished
    -- No --> Current[Add to currently in-progress tasks]
    --> Issue2[Open issue comment editor]
    ```
