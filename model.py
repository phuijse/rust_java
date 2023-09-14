import torch
import torch.nn as nn


class MyModel(nn.Module):

    def __init__(self):
        super(type(self), self).__init__()
        self.hidden_layers = nn.ModuleList([nn.Linear(2, 5), nn.Linear(5, 5)])
        self.output_layer = nn.Linear(5, 1)
        self.activation = nn.ReLU()

    def forward(self, x):
        for layer in self.hidden_layers:
            x = self.activation(layer(x))
        return self.output_layer(x)


if __name__ == "__main__":
    model = MyModel()
    traced_module = torch.jit.trace(model, torch.Tensor([1.0, 1.0]))
    traced_module.save("traced_model.pt")


