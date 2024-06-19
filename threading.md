# Preemptive vs Cooperative Multitasking

| Feature                    | Preemptive Multitasking                           | Cooperative Multitasking                          |
|----------------------------|--------------------------------------------------|--------------------------------------------------|
| **Definition**             | OS scheduler decides when to switch between tasks | Tasks voluntarily yield control to switch        |
| **Control**                | Operating system                                 | Individual tasks                                 |
| **Responsiveness**         | High                                             | Can be low if tasks don't yield control          |
| **Fairness**               | High                                             | Can be low, depending on task behavior           |
| **Complexity**             | Higher (due to context switching and synchronization) | Lower (simpler task management)                  |
| **Context Switching**      | Handled by OS, can occur at any time             | Handled by tasks, occurs at well-defined points  |
| **Risk of Starvation**     | Low, as OS ensures fair CPU time allocation      | High, if tasks do not yield control              |
| **Common Usage**           | Modern operating systems (Windows, macOS, Linux) | Older operating systems (e.g., Windows 3.x, classic Mac OS) |
| **Advantages**             | Fairness, responsiveness, better resource utilization | Simplicity, predictability                       |
| **Disadvantages**          | Higher complexity, potential for race conditions | Potential for unresponsive system if tasks misbehave |
| **Example Systems**        | Windows 10, macOS, Linux                         | Windows 3.x, classic Mac OS                      |

## Summary

- **Preemptive Multitasking**: Used in most modern operating systems, ensuring that all tasks get fair CPU time by allowing the OS to preempt tasks at any time.
- **Cooperative Multitasking**: Relies on tasks to voluntarily yield control, leading to simpler implementation but potential issues with fairness and responsiveness if tasks do not cooperate properly.
