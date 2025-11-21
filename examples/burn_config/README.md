# Burn Neural Network Configuration Example

This example demonstrates using Fusabi scripts to define neural network architectures for the Burn deep learning framework.

## Overview

Define neural network models using F# scripts instead of hardcoding architectures in Rust. This enables:
- Rapid prototyping of model architectures
- Configuration-driven model development
- Sharing model definitions across projects
- Non-ML engineers can adjust hyperparameters

## Prerequisites

- Rust 1.70+
- CUDA/Metal/CPU backend for Burn

## Running the Example

```bash
cd examples/burn_config
cargo run
```

## How It Works

### 1. Model Definition
The `model_config.fsx` file contains F# records describing:
- Network layers and their parameters
- Optimizer configuration
- Training hyperparameters
- Data preprocessing settings

### 2. Script Parsing
The application:
- Loads the configuration script
- Extracts model parameters
- Builds a Burn model dynamically

### 3. Model Creation
Based on the configuration:
- Layers are instantiated
- Optimizers are configured
- Training loop is set up

## Configuration Structure

### Layer Types

```fsharp
// Convolutional layer
{
    type = "conv2d"
    filters = 32
    kernelSize = 3
    stride = 1
    padding = 1
    activation = "relu"
}

// Dense/Linear layer
{
    type = "linear"
    outputSize = 128
    activation = "relu"
    dropout = 0.5
}

// Pooling layer
{
    type = "maxpool2d"
    poolSize = 2
    stride = 2
}
```

### Activation Functions
- `"relu"` - Rectified Linear Unit
- `"sigmoid"` - Sigmoid activation
- `"tanh"` - Hyperbolic tangent
- `"softmax"` - Softmax for classification
- `"gelu"` - Gaussian Error Linear Unit
- `"swish"` - Swish activation

### Optimizer Configuration

```fsharp
optimizer = {
    type = "adam"
    learningRate = 0.001
    betas = (0.9, 0.999)
    epsilon = 1e-08
    weightDecay = 0.0001
}
```

Supported optimizers:
- `"sgd"` - Stochastic Gradient Descent
- `"adam"` - Adam optimizer
- `"adamw"` - AdamW with weight decay
- `"rmsprop"` - RMSprop optimizer

## Example Architectures

### CNN for Image Classification
```fsharp
let cnn = {
    layers = [
        { type = "conv2d"; filters = 32; kernelSize = 3 }
        { type = "relu" }
        { type = "maxpool2d"; poolSize = 2 }
        { type = "conv2d"; filters = 64; kernelSize = 3 }
        { type = "relu" }
        { type = "maxpool2d"; poolSize = 2 }
        { type = "flatten" }
        { type = "linear"; outputSize = 128 }
        { type = "relu" }
        { type = "dropout"; rate = 0.5 }
        { type = "linear"; outputSize = 10 }
    ]
}
```

### MLP for Tabular Data
```fsharp
let mlp = {
    layers = [
        { type = "linear"; outputSize = 256; activation = "relu" }
        { type = "batchNorm" }
        { type = "dropout"; rate = 0.3 }
        { type = "linear"; outputSize = 128; activation = "relu" }
        { type = "batchNorm" }
        { type = "dropout"; rate = 0.3 }
        { type = "linear"; outputSize = 64; activation = "relu" }
        { type = "linear"; outputSize = 1; activation = "sigmoid" }
    ]
}
```

### Autoencoder
```fsharp
let autoencoder = {
    encoder = [
        { type = "linear"; outputSize = 128 }
        { type = "relu" }
        { type = "linear"; outputSize = 64 }
        { type = "relu" }
        { type = "linear"; outputSize = 32 }  // Latent space
    ]
    decoder = [
        { type = "linear"; outputSize = 64 }
        { type = "relu" }
        { type = "linear"; outputSize = 128 }
        { type = "relu" }
        { type = "linear"; outputSize = 784 }  // Original size
        { type = "sigmoid" }
    ]
}
```

## Training Configuration

### Basic Training Loop
```fsharp
let training = {
    epochs = 100
    batchSize = 32
    validationSplit = 0.2
    shuffle = true
    seed = 42
}
```

### Advanced Features
```fsharp
let advanced = {
    // Early stopping
    earlyStopping = {
        enabled = true
        patience = 10
        minDelta = 0.001
        monitor = "val_loss"
    }

    // Learning rate scheduling
    scheduler = {
        type = "cosineAnnealing"
        tMax = 100
        etaMin = 0.00001
    }

    // Gradient clipping
    gradientClipping = {
        enabled = true
        maxNorm = 1.0
    }

    // Mixed precision training
    mixedPrecision = true
}
```

## Data Augmentation

```fsharp
let augmentation = {
    // Image augmentation
    randomFlip = "horizontal"
    randomRotation = 15  // degrees
    randomZoom = 0.1
    randomBrightness = 0.2

    // Noise injection
    gaussianNoise = 0.01

    // Cutout/Mixup
    cutout = { enabled = true; size = 16 }
    mixup = { enabled = true; alpha = 0.2 }
}
```

## Metrics and Logging

```fsharp
let metrics = {
    // Classification metrics
    accuracy = true
    precision = true
    recall = true
    f1Score = true
    confusionMatrix = true

    // Regression metrics
    mse = true
    mae = true
    r2Score = true

    // Logging
    tensorboard = true
    wandb = { enabled = true; project = "fusabi-burn" }
}
```

## Integration with Burn

The example shows how to:
1. Parse F# configuration
2. Build Burn modules dynamically
3. Configure optimizers
4. Set up training loops
5. Apply data transformations

## Performance Considerations

- Model compilation happens once at startup
- Configuration parsing is cached
- Use smaller batch sizes for testing
- Enable GPU acceleration when available

## Future Enhancements

When WS1 (HOF Support) is complete:
- Dynamic layer composition with functions
- Custom loss functions in F#
- Metric calculation in scripts
- Callback functions for training events

## Troubleshooting

### Configuration Not Loading
- Check `model_config.fsx` syntax
- Verify all required fields present
- Check console for parsing errors

### Model Creation Fails
- Ensure layer dimensions match
- Verify activation functions are valid
- Check dropout rates are between 0 and 1

### Training Issues
- Adjust learning rate
- Check data normalization
- Monitor gradient norms

## Related Examples

- `bevy_scripting`: Game logic configuration
- `web_server`: Validation rules
- `ratatui_layout`: UI configuration