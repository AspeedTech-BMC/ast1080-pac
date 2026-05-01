#[doc = "Register `SCU2B0` reader"]
pub type R = crate::R<Scu2b0Spec>;
#[doc = "Register `SCU2B0` writer"]
pub type W = crate::W<Scu2b0Spec>;
#[doc = "Field `SCUCLKSEL2LOCK0` reader - SCU_CLK_SEL2_LOCK_0"]
pub type Scuclksel2lock0R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2LOCK0` writer - SCU_CLK_SEL2_LOCK_0"]
pub type Scuclksel2lock0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2LOCK3` reader - SCU_CLK_SEL2_LOCK_3"]
pub type Scuclksel2lock3R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2LOCK3` writer - SCU_CLK_SEL2_LOCK_3"]
pub type Scuclksel2lock3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2LOCK5` reader - SCU_CLK_SEL2_LOCK_5"]
pub type Scuclksel2lock5R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2LOCK5` writer - SCU_CLK_SEL2_LOCK_5"]
pub type Scuclksel2lock5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2LOCK8` reader - SCU_CLK_SEL2_LOCK_8"]
pub type Scuclksel2lock8R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2LOCK8` writer - SCU_CLK_SEL2_LOCK_8"]
pub type Scuclksel2lock8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2LOCK12` reader - SCU_CLK_SEL2_LOCK_12"]
pub type Scuclksel2lock12R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2LOCK12` writer - SCU_CLK_SEL2_LOCK_12"]
pub type Scuclksel2lock12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2LOCK15` reader - SCU_CLK_SEL2_LOCK_15"]
pub type Scuclksel2lock15R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2LOCK15` writer - SCU_CLK_SEL2_LOCK_15"]
pub type Scuclksel2lock15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2LOCK16` reader - SCU_CLK_SEL2_LOCK_16"]
pub type Scuclksel2lock16R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2LOCK16` writer - SCU_CLK_SEL2_LOCK_16"]
pub type Scuclksel2lock16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2LOCK17` reader - SCU_CLK_SEL2_LOCK_17"]
pub type Scuclksel2lock17R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2LOCK17` writer - SCU_CLK_SEL2_LOCK_17"]
pub type Scuclksel2lock17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2LOCK18` reader - SCU_CLK_SEL2_LOCK_18"]
pub type Scuclksel2lock18R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2LOCK18` writer - SCU_CLK_SEL2_LOCK_18"]
pub type Scuclksel2lock18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2LOCK19` reader - SCU_CLK_SEL2_LOCK_19"]
pub type Scuclksel2lock19R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2LOCK19` writer - SCU_CLK_SEL2_LOCK_19"]
pub type Scuclksel2lock19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2LOCK20` reader - SCU_CLK_SEL2_LOCK_20"]
pub type Scuclksel2lock20R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2LOCK20` writer - SCU_CLK_SEL2_LOCK_20"]
pub type Scuclksel2lock20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2LOCK23` reader - SCU_CLK_SEL2_LOCK_23"]
pub type Scuclksel2lock23R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2LOCK23` writer - SCU_CLK_SEL2_LOCK_23"]
pub type Scuclksel2lock23W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_CLK_SEL2_LOCK_0"]
    #[inline(always)]
    pub fn scuclksel2lock0(&self) -> Scuclksel2lock0R {
        Scuclksel2lock0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - SCU_CLK_SEL2_LOCK_3"]
    #[inline(always)]
    pub fn scuclksel2lock3(&self) -> Scuclksel2lock3R {
        Scuclksel2lock3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_CLK_SEL2_LOCK_5"]
    #[inline(always)]
    pub fn scuclksel2lock5(&self) -> Scuclksel2lock5R {
        Scuclksel2lock5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SCU_CLK_SEL2_LOCK_8"]
    #[inline(always)]
    pub fn scuclksel2lock8(&self) -> Scuclksel2lock8R {
        Scuclksel2lock8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - SCU_CLK_SEL2_LOCK_12"]
    #[inline(always)]
    pub fn scuclksel2lock12(&self) -> Scuclksel2lock12R {
        Scuclksel2lock12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - SCU_CLK_SEL2_LOCK_15"]
    #[inline(always)]
    pub fn scuclksel2lock15(&self) -> Scuclksel2lock15R {
        Scuclksel2lock15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_CLK_SEL2_LOCK_16"]
    #[inline(always)]
    pub fn scuclksel2lock16(&self) -> Scuclksel2lock16R {
        Scuclksel2lock16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_CLK_SEL2_LOCK_17"]
    #[inline(always)]
    pub fn scuclksel2lock17(&self) -> Scuclksel2lock17R {
        Scuclksel2lock17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_CLK_SEL2_LOCK_18"]
    #[inline(always)]
    pub fn scuclksel2lock18(&self) -> Scuclksel2lock18R {
        Scuclksel2lock18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_CLK_SEL2_LOCK_19"]
    #[inline(always)]
    pub fn scuclksel2lock19(&self) -> Scuclksel2lock19R {
        Scuclksel2lock19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SCU_CLK_SEL2_LOCK_20"]
    #[inline(always)]
    pub fn scuclksel2lock20(&self) -> Scuclksel2lock20R {
        Scuclksel2lock20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - SCU_CLK_SEL2_LOCK_23"]
    #[inline(always)]
    pub fn scuclksel2lock23(&self) -> Scuclksel2lock23R {
        Scuclksel2lock23R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_CLK_SEL2_LOCK_0"]
    #[inline(always)]
    pub fn scuclksel2lock0(&mut self) -> Scuclksel2lock0W<Scu2b0Spec> {
        Scuclksel2lock0W::new(self, 0)
    }
    #[doc = "Bit 3 - SCU_CLK_SEL2_LOCK_3"]
    #[inline(always)]
    pub fn scuclksel2lock3(&mut self) -> Scuclksel2lock3W<Scu2b0Spec> {
        Scuclksel2lock3W::new(self, 3)
    }
    #[doc = "Bit 5 - SCU_CLK_SEL2_LOCK_5"]
    #[inline(always)]
    pub fn scuclksel2lock5(&mut self) -> Scuclksel2lock5W<Scu2b0Spec> {
        Scuclksel2lock5W::new(self, 5)
    }
    #[doc = "Bit 8 - SCU_CLK_SEL2_LOCK_8"]
    #[inline(always)]
    pub fn scuclksel2lock8(&mut self) -> Scuclksel2lock8W<Scu2b0Spec> {
        Scuclksel2lock8W::new(self, 8)
    }
    #[doc = "Bit 12 - SCU_CLK_SEL2_LOCK_12"]
    #[inline(always)]
    pub fn scuclksel2lock12(&mut self) -> Scuclksel2lock12W<Scu2b0Spec> {
        Scuclksel2lock12W::new(self, 12)
    }
    #[doc = "Bit 15 - SCU_CLK_SEL2_LOCK_15"]
    #[inline(always)]
    pub fn scuclksel2lock15(&mut self) -> Scuclksel2lock15W<Scu2b0Spec> {
        Scuclksel2lock15W::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_CLK_SEL2_LOCK_16"]
    #[inline(always)]
    pub fn scuclksel2lock16(&mut self) -> Scuclksel2lock16W<Scu2b0Spec> {
        Scuclksel2lock16W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_CLK_SEL2_LOCK_17"]
    #[inline(always)]
    pub fn scuclksel2lock17(&mut self) -> Scuclksel2lock17W<Scu2b0Spec> {
        Scuclksel2lock17W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_CLK_SEL2_LOCK_18"]
    #[inline(always)]
    pub fn scuclksel2lock18(&mut self) -> Scuclksel2lock18W<Scu2b0Spec> {
        Scuclksel2lock18W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_CLK_SEL2_LOCK_19"]
    #[inline(always)]
    pub fn scuclksel2lock19(&mut self) -> Scuclksel2lock19W<Scu2b0Spec> {
        Scuclksel2lock19W::new(self, 19)
    }
    #[doc = "Bit 20 - SCU_CLK_SEL2_LOCK_20"]
    #[inline(always)]
    pub fn scuclksel2lock20(&mut self) -> Scuclksel2lock20W<Scu2b0Spec> {
        Scuclksel2lock20W::new(self, 20)
    }
    #[doc = "Bit 23 - SCU_CLK_SEL2_LOCK_23"]
    #[inline(always)]
    pub fn scuclksel2lock23(&mut self) -> Scuclksel2lock23W<Scu2b0Spec> {
        Scuclksel2lock23W::new(self, 23)
    }
}
#[doc = "Clock Selection Lock 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu2b0Spec;
impl crate::RegisterSpec for Scu2b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu2b0::R`](R) reader structure"]
impl crate::Readable for Scu2b0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu2b0::W`](W) writer structure"]
impl crate::Writable for Scu2b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU2B0 to value 0"]
impl crate::Resettable for Scu2b0Spec {}
