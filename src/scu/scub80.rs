#[doc = "Register `SCUB80` reader"]
pub type R = crate::R<Scub80Spec>;
#[doc = "Register `SCUB80` writer"]
pub type W = crate::W<Scub80Spec>;
#[doc = "Field `SCUDISMCUPUF0` reader - SCU_DIS_MCU_PUF_0"]
pub type Scudismcupuf0R = crate::BitReader;
#[doc = "Field `SCUDISMCUPUF0` writer - SCU_DIS_MCU_PUF_0"]
pub type Scudismcupuf0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDISMCUPUF1` reader - SCU_DIS_MCU_PUF_1"]
pub type Scudismcupuf1R = crate::BitReader;
#[doc = "Field `SCUDISMCUPUF1` writer - SCU_DIS_MCU_PUF_1"]
pub type Scudismcupuf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDISMCUPUF2` reader - SCU_DIS_MCU_PUF_2"]
pub type Scudismcupuf2R = crate::BitReader;
#[doc = "Field `SCUDISMCUPUF2` writer - SCU_DIS_MCU_PUF_2"]
pub type Scudismcupuf2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_MCU_PUF_0"]
    #[inline(always)]
    pub fn scudismcupuf0(&self) -> Scudismcupuf0R {
        Scudismcupuf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_DIS_MCU_PUF_1"]
    #[inline(always)]
    pub fn scudismcupuf1(&self) -> Scudismcupuf1R {
        Scudismcupuf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_MCU_PUF_2"]
    #[inline(always)]
    pub fn scudismcupuf2(&self) -> Scudismcupuf2R {
        Scudismcupuf2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_MCU_PUF_0"]
    #[inline(always)]
    pub fn scudismcupuf0(&mut self) -> Scudismcupuf0W<Scub80Spec> {
        Scudismcupuf0W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_DIS_MCU_PUF_1"]
    #[inline(always)]
    pub fn scudismcupuf1(&mut self) -> Scudismcupuf1W<Scub80Spec> {
        Scudismcupuf1W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_DIS_MCU_PUF_2"]
    #[inline(always)]
    pub fn scudismcupuf2(&mut self) -> Scudismcupuf2W<Scub80Spec> {
        Scudismcupuf2W::new(self, 2)
    }
}
#[doc = "PUF Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scub80::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub80::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub80Spec;
impl crate::RegisterSpec for Scub80Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub80::R`](R) reader structure"]
impl crate::Readable for Scub80Spec {}
#[doc = "`write(|w| ..)` method takes [`scub80::W`](W) writer structure"]
impl crate::Writable for Scub80Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB80 to value 0"]
impl crate::Resettable for Scub80Spec {}
