MAX = 99999999999999

class Graph:
    def __init__(self):
        self.nodes = []
        self.container = {}

    def add_node(self, node):
        return self.nodes.append(node)

    def add_edge(self, from_node, to_node, weight):
        if from_node in self.container:
            self.container[from_node].update({to_node: weight})
        else:
            self.container[from_node] = {to_node: weight}

    def shortest_distance(self, origin):
        initial_paths = {origin: 0}
        for node in self.nodes:
            if node != origin:
                initial_paths[node] = MAX

        return initial_paths

g = Graph()

g.add_node('P')
g.add_node('R')
g.add_node('Z')
g.add_node('A')
g.add_node('Y')

g.add_edge('P', 'A', 105)
g.add_edge('P', 'Y', 1)
g.add_edge('P', 'R', 452)

g.add_edge('R', 'P', 10)
g.add_edge('R', 'Z', 192)

g.add_edge('Z', 'P', 12)

g.add_edge('A', 'R', 112)
g.add_edge('A', 'Y', 1000)

g.add_edge('Y', 'R', 2000)


def dijkstra(origin, destination, graph):
    shortest_distance = g.shortest_distance(origin)
    previous = {}
    path = []

    while graph:

        next_node = min(shortest_distance, key=shortest_distance.get)

        for adjacent, weight in graph[next_node].items():

            if adjacent in shortest_distance:
                if weight + shortest_distance[next_node] < shortest_distance[adjacent]:
                    shortest_distance[adjacent] = weight + shortest_distance[next_node]
                    previous[adjacent] = next_node

        shortest_distance.pop(next_node)
        graph.pop(next_node)

    current = destination

    while current != origin:
        path.insert(0, current)
        current = previous[current]

    path.insert(0, origin)

    print(path)

dijkstra("A", "P", g.container)
