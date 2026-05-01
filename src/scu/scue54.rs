#[doc = "Register `SCUE54` reader"]
pub type R = crate::R<Scue54Spec>;
#[doc = "Register `SCUE54` writer"]
pub type W = crate::W<Scue54Spec>;
#[doc = "Field `SCUREGLOCKA80` reader - SCU_REG_LOCK_A80"]
pub type Scureglocka80R = crate::BitReader;
#[doc = "Field `SCUREGLOCKA80` writer - SCU_REG_LOCK_A80"]
pub type Scureglocka80W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKA84` reader - SCU_REG_LOCK_A84"]
pub type Scureglocka84R = crate::BitReader;
#[doc = "Field `SCUREGLOCKA84` writer - SCU_REG_LOCK_A84"]
pub type Scureglocka84W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKA88` reader - SCU_REG_LOCK_A88"]
pub type Scureglocka88R = crate::BitReader;
#[doc = "Field `SCUREGLOCKA88` writer - SCU_REG_LOCK_A88"]
pub type Scureglocka88W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKA8C` reader - SCU_REG_LOCK_A8C"]
pub type Scureglocka8cR = crate::BitReader;
#[doc = "Field `SCUREGLOCKA8C` writer - SCU_REG_LOCK_A8C"]
pub type Scureglocka8cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKA90` reader - SCU_REG_LOCK_A90"]
pub type Scureglocka90R = crate::BitReader;
#[doc = "Field `SCUREGLOCKA90` writer - SCU_REG_LOCK_A90"]
pub type Scureglocka90W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKA94` reader - SCU_REG_LOCK_A94"]
pub type Scureglocka94R = crate::BitReader;
#[doc = "Field `SCUREGLOCKA94` writer - SCU_REG_LOCK_A94"]
pub type Scureglocka94W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKA98` reader - SCU_REG_LOCK_A98"]
pub type Scureglocka98R = crate::BitReader;
#[doc = "Field `SCUREGLOCKA98` writer - SCU_REG_LOCK_A98"]
pub type Scureglocka98W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKA9C` reader - SCU_REG_LOCK_A9C"]
pub type Scureglocka9cR = crate::BitReader;
#[doc = "Field `SCUREGLOCKA9C` writer - SCU_REG_LOCK_A9C"]
pub type Scureglocka9cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKAA0` reader - SCU_REG_LOCK_AA0"]
pub type Scureglockaa0R = crate::BitReader;
#[doc = "Field `SCUREGLOCKAA0` writer - SCU_REG_LOCK_AA0"]
pub type Scureglockaa0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKAA4` reader - SCU_REG_LOCK_AA4"]
pub type Scureglockaa4R = crate::BitReader;
#[doc = "Field `SCUREGLOCKAA4` writer - SCU_REG_LOCK_AA4"]
pub type Scureglockaa4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKAA8` reader - SCU_REG_LOCK_AA8"]
pub type Scureglockaa8R = crate::BitReader;
#[doc = "Field `SCUREGLOCKAA8` writer - SCU_REG_LOCK_AA8"]
pub type Scureglockaa8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKAAC` reader - SCU_REG_LOCK_AAC"]
pub type ScureglockaacR = crate::BitReader;
#[doc = "Field `SCUREGLOCKAAC` writer - SCU_REG_LOCK_AAC"]
pub type ScureglockaacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKAB0` reader - SCU_REG_LOCK_AB0"]
pub type Scureglockab0R = crate::BitReader;
#[doc = "Field `SCUREGLOCKAB0` writer - SCU_REG_LOCK_AB0"]
pub type Scureglockab0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKAB4` reader - SCU_REG_LOCK_AB4"]
pub type Scureglockab4R = crate::BitReader;
#[doc = "Field `SCUREGLOCKAB4` writer - SCU_REG_LOCK_AB4"]
pub type Scureglockab4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKAB8` reader - SCU_REG_LOCK_AB8"]
pub type Scureglockab8R = crate::BitReader;
#[doc = "Field `SCUREGLOCKAB8` writer - SCU_REG_LOCK_AB8"]
pub type Scureglockab8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKABC` reader - SCU_REG_LOCK_ABC"]
pub type ScureglockabcR = crate::BitReader;
#[doc = "Field `SCUREGLOCKABC` writer - SCU_REG_LOCK_ABC"]
pub type ScureglockabcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_LOCK_A80"]
    #[inline(always)]
    pub fn scureglocka80(&self) -> Scureglocka80R {
        Scureglocka80R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_A84"]
    #[inline(always)]
    pub fn scureglocka84(&self) -> Scureglocka84R {
        Scureglocka84R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_A88"]
    #[inline(always)]
    pub fn scureglocka88(&self) -> Scureglocka88R {
        Scureglocka88R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_A8C"]
    #[inline(always)]
    pub fn scureglocka8c(&self) -> Scureglocka8cR {
        Scureglocka8cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_A90"]
    #[inline(always)]
    pub fn scureglocka90(&self) -> Scureglocka90R {
        Scureglocka90R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_A94"]
    #[inline(always)]
    pub fn scureglocka94(&self) -> Scureglocka94R {
        Scureglocka94R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_A98"]
    #[inline(always)]
    pub fn scureglocka98(&self) -> Scureglocka98R {
        Scureglocka98R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_A9C"]
    #[inline(always)]
    pub fn scureglocka9c(&self) -> Scureglocka9cR {
        Scureglocka9cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_AA0"]
    #[inline(always)]
    pub fn scureglockaa0(&self) -> Scureglockaa0R {
        Scureglockaa0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_REG_LOCK_AA4"]
    #[inline(always)]
    pub fn scureglockaa4(&self) -> Scureglockaa4R {
        Scureglockaa4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SCU_REG_LOCK_AA8"]
    #[inline(always)]
    pub fn scureglockaa8(&self) -> Scureglockaa8R {
        Scureglockaa8R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SCU_REG_LOCK_AAC"]
    #[inline(always)]
    pub fn scureglockaac(&self) -> ScureglockaacR {
        ScureglockaacR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCU_REG_LOCK_AB0"]
    #[inline(always)]
    pub fn scureglockab0(&self) -> Scureglockab0R {
        Scureglockab0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_LOCK_AB4"]
    #[inline(always)]
    pub fn scureglockab4(&self) -> Scureglockab4R {
        Scureglockab4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_LOCK_AB8"]
    #[inline(always)]
    pub fn scureglockab8(&self) -> Scureglockab8R {
        Scureglockab8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_LOCK_ABC"]
    #[inline(always)]
    pub fn scureglockabc(&self) -> ScureglockabcR {
        ScureglockabcR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_LOCK_A80"]
    #[inline(always)]
    pub fn scureglocka80(&mut self) -> Scureglocka80W<Scue54Spec> {
        Scureglocka80W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_A84"]
    #[inline(always)]
    pub fn scureglocka84(&mut self) -> Scureglocka84W<Scue54Spec> {
        Scureglocka84W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_A88"]
    #[inline(always)]
    pub fn scureglocka88(&mut self) -> Scureglocka88W<Scue54Spec> {
        Scureglocka88W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_A8C"]
    #[inline(always)]
    pub fn scureglocka8c(&mut self) -> Scureglocka8cW<Scue54Spec> {
        Scureglocka8cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_A90"]
    #[inline(always)]
    pub fn scureglocka90(&mut self) -> Scureglocka90W<Scue54Spec> {
        Scureglocka90W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_A94"]
    #[inline(always)]
    pub fn scureglocka94(&mut self) -> Scureglocka94W<Scue54Spec> {
        Scureglocka94W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_A98"]
    #[inline(always)]
    pub fn scureglocka98(&mut self) -> Scureglocka98W<Scue54Spec> {
        Scureglocka98W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_A9C"]
    #[inline(always)]
    pub fn scureglocka9c(&mut self) -> Scureglocka9cW<Scue54Spec> {
        Scureglocka9cW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_AA0"]
    #[inline(always)]
    pub fn scureglockaa0(&mut self) -> Scureglockaa0W<Scue54Spec> {
        Scureglockaa0W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_REG_LOCK_AA4"]
    #[inline(always)]
    pub fn scureglockaa4(&mut self) -> Scureglockaa4W<Scue54Spec> {
        Scureglockaa4W::new(self, 9)
    }
    #[doc = "Bit 10 - SCU_REG_LOCK_AA8"]
    #[inline(always)]
    pub fn scureglockaa8(&mut self) -> Scureglockaa8W<Scue54Spec> {
        Scureglockaa8W::new(self, 10)
    }
    #[doc = "Bit 11 - SCU_REG_LOCK_AAC"]
    #[inline(always)]
    pub fn scureglockaac(&mut self) -> ScureglockaacW<Scue54Spec> {
        ScureglockaacW::new(self, 11)
    }
    #[doc = "Bit 12 - SCU_REG_LOCK_AB0"]
    #[inline(always)]
    pub fn scureglockab0(&mut self) -> Scureglockab0W<Scue54Spec> {
        Scureglockab0W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_LOCK_AB4"]
    #[inline(always)]
    pub fn scureglockab4(&mut self) -> Scureglockab4W<Scue54Spec> {
        Scureglockab4W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_LOCK_AB8"]
    #[inline(always)]
    pub fn scureglockab8(&mut self) -> Scureglockab8W<Scue54Spec> {
        Scureglockab8W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_LOCK_ABC"]
    #[inline(always)]
    pub fn scureglockabc(&mut self) -> ScureglockabcW<Scue54Spec> {
        ScureglockabcW::new(self, 15)
    }
}
#[doc = "Write Protection 22 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue54Spec;
impl crate::RegisterSpec for Scue54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue54::R`](R) reader structure"]
impl crate::Readable for Scue54Spec {}
#[doc = "`write(|w| ..)` method takes [`scue54::W`](W) writer structure"]
impl crate::Writable for Scue54Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE54 to value 0"]
impl crate::Resettable for Scue54Spec {}
