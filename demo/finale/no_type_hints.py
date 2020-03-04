N = 1000
iterations = 100
eps = 0.00125
dmp = 0.995
dt = 0.001

seed = 42
modulus = 8191
multiplier = 8121
increment = 7

x = []
y = []
z = []

ax = []
ay = []
az = []

vx = []
vy = []
vz = []

m = []

def abs(x: int):
    if x < 0:
        return -x

    return x

def random_value() -> float:
    global seed
    seed = abs(multiplier * seed + increment) % modulus
    return seed

def initialise():
    initialise_global_lists()
    i = 0

    while i < N:
        x.append(random_value() / modulus)
        y.append(random_value() / modulus)
        z.append(random_value() / modulus)

        ax.append(0.0)
        ay.append(0.0)
        az.append(0.0)

        vx.append(random_value() / modulus)
        vy.append(random_value() / modulus)
        vz.append(random_value() / modulus)

        m.append(random_value() / modulus)

        i += 1

def square_root(n):
    root = n / 2
    temp = 0.0

    while root != temp:
        temp = root
        root = (n / temp + temp) / 2

    return root

def power(n, i):
    res: float = n
    iterations = 0

    while iterations < i:
        res *= n
        iterations += 1

    return res

def compute():
    i = 0
    j = 0

    while i < N:
        ax[i] = 0.0
        ay[i] = 0.0
        az[i] = 0.0

        i += 1

    i = 0
    j = 0

    while i < N:
        while j < N:
            rx = x[j] - x[i]
            ry = y[j] - y[i]
            rz = z[j] - z[i]
            r2 = rx*rx + ry*ry + rz*rz + eps
            r2inv = 1.0 / square_root(r2)
            r6inv = power(r2inv, 3)
            s = m[j] * r6inv
            ax[i] += s * rx
            ay[i] += s * ry
            az[i] += s * rz

            j += 1

        i += 1

    i = 0

    while i < N:
        vx[i] += dmp * (dt * ax[i])
        vy[i] += dmp * (dt * ay[i])
        vz[i] += dmp * (dt * az[i])

        i += 1

    i = 0

    while i < N:
        x[i] = dt * vx[i]
        y[i] = dt * vy[i]
        z[i] = dt * vz[i]

        if x[i] >= 1.0 or x[i] <= -1.0:
            vx[i] *= -1.0

        if y[i] >= 1.0 or y[i] <= -1.0:
            vy[i] *= -1.0

        if z[i] >= 1.0 or z[i] <= -1.0:
            vz[i] *= -1.0

        i += 1

def verify():
    phi = 0.0
    i = 0
    j = 0

    while i < N:
        j = 0
        while j < N:
            rx = x[j] - x[i]
            ry = y[j] - y[i]
            rz = z[j] - z[i]
            r2 = rx*rx + ry*ry + rz*rz + eps
            r2inv = 1.0 / square_root(r2)
            r6inv = power(r2inv, 3)
            phi += m[j] * r6inv
            j += 1
        i += 1

    return phi

def calculate():
    initialise()

    i = 0

    while i < iterations:
        compute()
        i += 1

    phi = verify()
    return phi
