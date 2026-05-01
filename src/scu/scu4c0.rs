#[doc = "Register `SCU4C0` reader"]
pub type R = crate::R<Scu4c0Spec>;
#[doc = "Register `SCU4C0` writer"]
pub type W = crate::W<Scu4c0Spec>;
#[doc = "Field `SCUDISPDIO032` reader - SCU_DIS_PD_IO032"]
pub type Scudispdio032R = crate::BitReader;
#[doc = "Field `SCUDISPDIO032` writer - SCU_DIS_PD_IO032"]
pub type Scudispdio032W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO032` reader - SCU_DIS_PU_IO032"]
pub type Scudispuio032R = crate::BitReader;
#[doc = "Field `SCUDISPUIO032` writer - SCU_DIS_PU_IO032"]
pub type Scudispuio032W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO032` reader - SCU_DRV_IO032"]
pub type Scudrvio032R = crate::FieldReader;
#[doc = "Field `SCUDRVIO032` writer - SCU_DRV_IO032"]
pub type Scudrvio032W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO032` reader - SCU_EN_SMT_IO032"]
pub type Scuensmtio032R = crate::BitReader;
#[doc = "Field `SCUENSMTIO032` writer - SCU_EN_SMT_IO032"]
pub type Scuensmtio032W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO032` reader - SCU_EN_HV_IO032"]
pub type Scuenhvio032R = crate::BitReader;
#[doc = "Field `SCUENHVIO032` writer - SCU_EN_HV_IO032"]
pub type Scuenhvio032W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO033` reader - SCU_DIS_PD_IO033"]
pub type Scudispdio033R = crate::BitReader;
#[doc = "Field `SCUDISPDIO033` writer - SCU_DIS_PD_IO033"]
pub type Scudispdio033W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO033` reader - SCU_DIS_PU_IO033"]
pub type Scudispuio033R = crate::BitReader;
#[doc = "Field `SCUDISPUIO033` writer - SCU_DIS_PU_IO033"]
pub type Scudispuio033W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO033` reader - SCU_DRV_IO033"]
pub type Scudrvio033R = crate::FieldReader;
#[doc = "Field `SCUDRVIO033` writer - SCU_DRV_IO033"]
pub type Scudrvio033W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO033` reader - SCU_EN_SMT_IO033"]
pub type Scuensmtio033R = crate::BitReader;
#[doc = "Field `SCUENSMTIO033` writer - SCU_EN_SMT_IO033"]
pub type Scuensmtio033W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO033` reader - SCU_EN_HV_IO033"]
pub type Scuenhvio033R = crate::BitReader;
#[doc = "Field `SCUENHVIO033` writer - SCU_EN_HV_IO033"]
pub type Scuenhvio033W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO032"]
    #[inline(always)]
    pub fn scudispdio032(&self) -> Scudispdio032R {
        Scudispdio032R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO032"]
    #[inline(always)]
    pub fn scudispuio032(&self) -> Scudispuio032R {
        Scudispuio032R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO032"]
    #[inline(always)]
    pub fn scudrvio032(&self) -> Scudrvio032R {
        Scudrvio032R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO032"]
    #[inline(always)]
    pub fn scuensmtio032(&self) -> Scuensmtio032R {
        Scuensmtio032R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO032"]
    #[inline(always)]
    pub fn scuenhvio032(&self) -> Scuenhvio032R {
        Scuenhvio032R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO033"]
    #[inline(always)]
    pub fn scudispdio033(&self) -> Scudispdio033R {
        Scudispdio033R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO033"]
    #[inline(always)]
    pub fn scudispuio033(&self) -> Scudispuio033R {
        Scudispuio033R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO033"]
    #[inline(always)]
    pub fn scudrvio033(&self) -> Scudrvio033R {
        Scudrvio033R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO033"]
    #[inline(always)]
    pub fn scuensmtio033(&self) -> Scuensmtio033R {
        Scuensmtio033R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO033"]
    #[inline(always)]
    pub fn scuenhvio033(&self) -> Scuenhvio033R {
        Scuenhvio033R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO032"]
    #[inline(always)]
    pub fn scudispdio032(&mut self) -> Scudispdio032W<Scu4c0Spec> {
        Scudispdio032W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO032"]
    #[inline(always)]
    pub fn scudispuio032(&mut self) -> Scudispuio032W<Scu4c0Spec> {
        Scudispuio032W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO032"]
    #[inline(always)]
    pub fn scudrvio032(&mut self) -> Scudrvio032W<Scu4c0Spec> {
        Scudrvio032W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO032"]
    #[inline(always)]
    pub fn scuensmtio032(&mut self) -> Scuensmtio032W<Scu4c0Spec> {
        Scuensmtio032W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO032"]
    #[inline(always)]
    pub fn scuenhvio032(&mut self) -> Scuenhvio032W<Scu4c0Spec> {
        Scuenhvio032W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO033"]
    #[inline(always)]
    pub fn scudispdio033(&mut self) -> Scudispdio033W<Scu4c0Spec> {
        Scudispdio033W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO033"]
    #[inline(always)]
    pub fn scudispuio033(&mut self) -> Scudispuio033W<Scu4c0Spec> {
        Scudispuio033W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO033"]
    #[inline(always)]
    pub fn scudrvio033(&mut self) -> Scudrvio033W<Scu4c0Spec> {
        Scudrvio033W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO033"]
    #[inline(always)]
    pub fn scuensmtio033(&mut self) -> Scuensmtio033W<Scu4c0Spec> {
        Scuensmtio033W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO033"]
    #[inline(always)]
    pub fn scuenhvio033(&mut self) -> Scuenhvio033W<Scu4c0Spec> {
        Scuenhvio033W::new(self, 25)
    }
}
#[doc = "IO Control \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4c0Spec;
impl crate::RegisterSpec for Scu4c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4c0::R`](R) reader structure"]
impl crate::Readable for Scu4c0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4c0::W`](W) writer structure"]
impl crate::Writable for Scu4c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4C0 to value 0x0204_0204"]
impl crate::Resettable for Scu4c0Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
