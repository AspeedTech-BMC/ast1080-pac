#[doc = "Register `SCU534` reader"]
pub type R = crate::R<Scu534Spec>;
#[doc = "Register `SCU534` writer"]
pub type W = crate::W<Scu534Spec>;
#[doc = "Field `SCUDISPDIO090` reader - SCU_DIS_PD_IO090"]
pub type Scudispdio090R = crate::BitReader;
#[doc = "Field `SCUDISPDIO090` writer - SCU_DIS_PD_IO090"]
pub type Scudispdio090W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO090` reader - SCU_DIS_PU_IO090"]
pub type Scudispuio090R = crate::BitReader;
#[doc = "Field `SCUDISPUIO090` writer - SCU_DIS_PU_IO090"]
pub type Scudispuio090W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO090` reader - SCU_DRV_IO090"]
pub type Scudrvio090R = crate::FieldReader;
#[doc = "Field `SCUDRVIO090` writer - SCU_DRV_IO090"]
pub type Scudrvio090W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO090` reader - SCU_EN_SMT_IO090"]
pub type Scuensmtio090R = crate::BitReader;
#[doc = "Field `SCUENSMTIO090` writer - SCU_EN_SMT_IO090"]
pub type Scuensmtio090W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO090` reader - SCU_EN_HV_IO090"]
pub type Scuenhvio090R = crate::BitReader;
#[doc = "Field `SCUENHVIO090` writer - SCU_EN_HV_IO090"]
pub type Scuenhvio090W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO091` reader - SCU_DIS_PD_IO091"]
pub type Scudispdio091R = crate::BitReader;
#[doc = "Field `SCUDISPDIO091` writer - SCU_DIS_PD_IO091"]
pub type Scudispdio091W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO091` reader - SCU_DIS_PU_IO091"]
pub type Scudispuio091R = crate::BitReader;
#[doc = "Field `SCUDISPUIO091` writer - SCU_DIS_PU_IO091"]
pub type Scudispuio091W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO091` reader - SCU_DRV_IO091"]
pub type Scudrvio091R = crate::FieldReader;
#[doc = "Field `SCUDRVIO091` writer - SCU_DRV_IO091"]
pub type Scudrvio091W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO091` reader - SCU_EN_SMT_IO091"]
pub type Scuensmtio091R = crate::BitReader;
#[doc = "Field `SCUENSMTIO091` writer - SCU_EN_SMT_IO091"]
pub type Scuensmtio091W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO091` reader - SCU_EN_HV_IO091"]
pub type Scuenhvio091R = crate::BitReader;
#[doc = "Field `SCUENHVIO091` writer - SCU_EN_HV_IO091"]
pub type Scuenhvio091W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO090"]
    #[inline(always)]
    pub fn scudispdio090(&self) -> Scudispdio090R {
        Scudispdio090R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO090"]
    #[inline(always)]
    pub fn scudispuio090(&self) -> Scudispuio090R {
        Scudispuio090R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO090"]
    #[inline(always)]
    pub fn scudrvio090(&self) -> Scudrvio090R {
        Scudrvio090R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO090"]
    #[inline(always)]
    pub fn scuensmtio090(&self) -> Scuensmtio090R {
        Scuensmtio090R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO090"]
    #[inline(always)]
    pub fn scuenhvio090(&self) -> Scuenhvio090R {
        Scuenhvio090R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO091"]
    #[inline(always)]
    pub fn scudispdio091(&self) -> Scudispdio091R {
        Scudispdio091R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO091"]
    #[inline(always)]
    pub fn scudispuio091(&self) -> Scudispuio091R {
        Scudispuio091R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO091"]
    #[inline(always)]
    pub fn scudrvio091(&self) -> Scudrvio091R {
        Scudrvio091R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO091"]
    #[inline(always)]
    pub fn scuensmtio091(&self) -> Scuensmtio091R {
        Scuensmtio091R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO091"]
    #[inline(always)]
    pub fn scuenhvio091(&self) -> Scuenhvio091R {
        Scuenhvio091R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO090"]
    #[inline(always)]
    pub fn scudispdio090(&mut self) -> Scudispdio090W<Scu534Spec> {
        Scudispdio090W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO090"]
    #[inline(always)]
    pub fn scudispuio090(&mut self) -> Scudispuio090W<Scu534Spec> {
        Scudispuio090W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO090"]
    #[inline(always)]
    pub fn scudrvio090(&mut self) -> Scudrvio090W<Scu534Spec> {
        Scudrvio090W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO090"]
    #[inline(always)]
    pub fn scuensmtio090(&mut self) -> Scuensmtio090W<Scu534Spec> {
        Scuensmtio090W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO090"]
    #[inline(always)]
    pub fn scuenhvio090(&mut self) -> Scuenhvio090W<Scu534Spec> {
        Scuenhvio090W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO091"]
    #[inline(always)]
    pub fn scudispdio091(&mut self) -> Scudispdio091W<Scu534Spec> {
        Scudispdio091W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO091"]
    #[inline(always)]
    pub fn scudispuio091(&mut self) -> Scudispuio091W<Scu534Spec> {
        Scudispuio091W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO091"]
    #[inline(always)]
    pub fn scudrvio091(&mut self) -> Scudrvio091W<Scu534Spec> {
        Scudrvio091W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO091"]
    #[inline(always)]
    pub fn scuensmtio091(&mut self) -> Scuensmtio091W<Scu534Spec> {
        Scuensmtio091W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO091"]
    #[inline(always)]
    pub fn scuenhvio091(&mut self) -> Scuenhvio091W<Scu534Spec> {
        Scuenhvio091W::new(self, 25)
    }
}
#[doc = "IO Control \\#46\n\nYou can [`read`](crate::Reg::read) this register and get [`scu534::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu534::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu534Spec;
impl crate::RegisterSpec for Scu534Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu534::R`](R) reader structure"]
impl crate::Readable for Scu534Spec {}
#[doc = "`write(|w| ..)` method takes [`scu534::W`](W) writer structure"]
impl crate::Writable for Scu534Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU534 to value 0x0204_0204"]
impl crate::Resettable for Scu534Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
