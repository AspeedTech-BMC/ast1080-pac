#[doc = "Register `SCU5FC` reader"]
pub type R = crate::R<Scu5fcSpec>;
#[doc = "Register `SCU5FC` writer"]
pub type W = crate::W<Scu5fcSpec>;
#[doc = "Field `SCUDISPDIO190` reader - SCU_DIS_PD_IO190"]
pub type Scudispdio190R = crate::BitReader;
#[doc = "Field `SCUDISPDIO190` writer - SCU_DIS_PD_IO190"]
pub type Scudispdio190W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO190` reader - SCU_DIS_PU_IO190"]
pub type Scudispuio190R = crate::BitReader;
#[doc = "Field `SCUDISPUIO190` writer - SCU_DIS_PU_IO190"]
pub type Scudispuio190W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO190` reader - SCU_DRV_IO190"]
pub type Scudrvio190R = crate::FieldReader;
#[doc = "Field `SCUDRVIO190` writer - SCU_DRV_IO190"]
pub type Scudrvio190W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO190` reader - SCU_EN_SMT_IO190"]
pub type Scuensmtio190R = crate::BitReader;
#[doc = "Field `SCUENSMTIO190` writer - SCU_EN_SMT_IO190"]
pub type Scuensmtio190W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO190` reader - SCU_EN_HV_IO190"]
pub type Scuenhvio190R = crate::BitReader;
#[doc = "Field `SCUENHVIO190` writer - SCU_EN_HV_IO190"]
pub type Scuenhvio190W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO191` reader - SCU_DIS_PD_IO191"]
pub type Scudispdio191R = crate::BitReader;
#[doc = "Field `SCUDISPDIO191` writer - SCU_DIS_PD_IO191"]
pub type Scudispdio191W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO191` reader - SCU_DIS_PU_IO191"]
pub type Scudispuio191R = crate::BitReader;
#[doc = "Field `SCUDISPUIO191` writer - SCU_DIS_PU_IO191"]
pub type Scudispuio191W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO191` reader - SCU_DRV_IO191"]
pub type Scudrvio191R = crate::FieldReader;
#[doc = "Field `SCUDRVIO191` writer - SCU_DRV_IO191"]
pub type Scudrvio191W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO191` reader - SCU_EN_SMT_IO191"]
pub type Scuensmtio191R = crate::BitReader;
#[doc = "Field `SCUENSMTIO191` writer - SCU_EN_SMT_IO191"]
pub type Scuensmtio191W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO191` reader - SCU_EN_HV_IO191"]
pub type Scuenhvio191R = crate::BitReader;
#[doc = "Field `SCUENHVIO191` writer - SCU_EN_HV_IO191"]
pub type Scuenhvio191W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO190"]
    #[inline(always)]
    pub fn scudispdio190(&self) -> Scudispdio190R {
        Scudispdio190R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO190"]
    #[inline(always)]
    pub fn scudispuio190(&self) -> Scudispuio190R {
        Scudispuio190R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO190"]
    #[inline(always)]
    pub fn scudrvio190(&self) -> Scudrvio190R {
        Scudrvio190R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO190"]
    #[inline(always)]
    pub fn scuensmtio190(&self) -> Scuensmtio190R {
        Scuensmtio190R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO190"]
    #[inline(always)]
    pub fn scuenhvio190(&self) -> Scuenhvio190R {
        Scuenhvio190R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO191"]
    #[inline(always)]
    pub fn scudispdio191(&self) -> Scudispdio191R {
        Scudispdio191R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO191"]
    #[inline(always)]
    pub fn scudispuio191(&self) -> Scudispuio191R {
        Scudispuio191R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO191"]
    #[inline(always)]
    pub fn scudrvio191(&self) -> Scudrvio191R {
        Scudrvio191R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO191"]
    #[inline(always)]
    pub fn scuensmtio191(&self) -> Scuensmtio191R {
        Scuensmtio191R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO191"]
    #[inline(always)]
    pub fn scuenhvio191(&self) -> Scuenhvio191R {
        Scuenhvio191R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO190"]
    #[inline(always)]
    pub fn scudispdio190(&mut self) -> Scudispdio190W<Scu5fcSpec> {
        Scudispdio190W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO190"]
    #[inline(always)]
    pub fn scudispuio190(&mut self) -> Scudispuio190W<Scu5fcSpec> {
        Scudispuio190W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO190"]
    #[inline(always)]
    pub fn scudrvio190(&mut self) -> Scudrvio190W<Scu5fcSpec> {
        Scudrvio190W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO190"]
    #[inline(always)]
    pub fn scuensmtio190(&mut self) -> Scuensmtio190W<Scu5fcSpec> {
        Scuensmtio190W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO190"]
    #[inline(always)]
    pub fn scuenhvio190(&mut self) -> Scuenhvio190W<Scu5fcSpec> {
        Scuenhvio190W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO191"]
    #[inline(always)]
    pub fn scudispdio191(&mut self) -> Scudispdio191W<Scu5fcSpec> {
        Scudispdio191W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO191"]
    #[inline(always)]
    pub fn scudispuio191(&mut self) -> Scudispuio191W<Scu5fcSpec> {
        Scudispuio191W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO191"]
    #[inline(always)]
    pub fn scudrvio191(&mut self) -> Scudrvio191W<Scu5fcSpec> {
        Scudrvio191W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO191"]
    #[inline(always)]
    pub fn scuensmtio191(&mut self) -> Scuensmtio191W<Scu5fcSpec> {
        Scuensmtio191W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO191"]
    #[inline(always)]
    pub fn scuenhvio191(&mut self) -> Scuenhvio191W<Scu5fcSpec> {
        Scuenhvio191W::new(self, 25)
    }
}
#[doc = "IO Control \\#96\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5fcSpec;
impl crate::RegisterSpec for Scu5fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5fc::R`](R) reader structure"]
impl crate::Readable for Scu5fcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu5fc::W`](W) writer structure"]
impl crate::Writable for Scu5fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5FC to value 0x0201_0201"]
impl crate::Resettable for Scu5fcSpec {
    const RESET_VALUE: u32 = 0x0201_0201;
}
