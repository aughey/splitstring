// use google tests
#include <gtest/gtest.h>
#include <vector>

class SAE
{
public:
    SAE(int site, int app, int entity) : site(site), app(app), entity(entity) {}

    // convert to an SAE from a vector<int>
    SAE(const std::vector<int>& vec)
    {
        if (vec.size() != 3)
        {
            site = app = entity = 0;
            return;
        }

        site = vec[0];
        app = vec[1];
        entity = vec[2];
    }

    int getSite() const { return site; }
    int getApp() const { return app; }
    int getEntity() const { return entity; }

private:
    int site;
    int app;
    int entity;
};

SAE set_from_vec(const std::vector<int> &vec) {
    return SAE(vec[0],vec[1],vec[2]);
}

TEST(VectorConversions, HappyPath)
{
    SAE sae(1, 2, 3);
    ASSERT_EQ(sae.getSite(), 1);
    ASSERT_EQ(sae.getApp(), 2);
    ASSERT_EQ(sae.getEntity(), 3);

    std::vector<int> vec = { 1, 2, 3 };
    SAE sae_vec = vec;

    ASSERT_EQ(sae_vec.getSite(), 1);
    ASSERT_EQ(sae_vec.getApp(), 2);
    ASSERT_EQ(sae_vec.getEntity(), 3);

    SAE sae_vec2 = set_from_vec(vec);

    ASSERT_EQ(sae_vec2.getSite(), 1);
    ASSERT_EQ(sae_vec2.getApp(), 2);
    ASSERT_EQ(sae_vec2.getEntity(), 3);
}
