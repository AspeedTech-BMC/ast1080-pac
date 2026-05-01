#[doc = "Register `SCU4F4` reader"]
pub type R = crate::R<Scu4f4Spec>;
#[doc = "Register `SCU4F4` writer"]
pub type W = crate::W<Scu4f4Spec>;
#[doc = "Field `SCUDISPDIO058` reader - SCU_DIS_PD_IO058"]
pub type Scudispdio058R = crate::BitReader;
#[doc = "Field `SCUDISPDIO058` writer - SCU_DIS_PD_IO058"]
pub type Scudispdio058W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO058` reader - SCU_DIS_PU_IO058"]
pub type Scudispuio058R = crate::BitReader;
#[doc = "Field `SCUDISPUIO058` writer - SCU_DIS_PU_IO058"]
pub type Scudispuio058W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO058` reader - SCU_DRV_IO058"]
pub type Scudrvio058R = crate::FieldReader;
#[doc = "Field `SCUDRVIO058` writer - SCU_DRV_IO058"]
pub type Scudrvio058W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO058` reader - SCU_EN_SMT_IO058"]
pub type Scuensmtio058R = crate::BitReader;
#[doc = "Field `SCUENSMTIO058` writer - SCU_EN_SMT_IO058"]
pub type Scuensmtio058W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO058` reader - SCU_EN_HV_IO058"]
pub type Scuenhvio058R = crate::BitReader;
#[doc = "Field `SCUENHVIO058` writer - SCU_EN_HV_IO058"]
pub type Scuenhvio058W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO059` reader - SCU_DIS_PD_IO059"]
pub type Scudispdio059R = crate::BitReader;
#[doc = "Field `SCUDISPDIO059` writer - SCU_DIS_PD_IO059"]
pub type Scudispdio059W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO059` reader - SCU_DIS_PU_IO059"]
pub type Scudispuio059R = crate::BitReader;
#[doc = "Field `SCUDISPUIO059` writer - SCU_DIS_PU_IO059"]
pub type Scudispuio059W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO059` reader - SCU_DRV_IO059"]
pub type Scudrvio059R = crate::FieldReader;
#[doc = "Field `SCUDRVIO059` writer - SCU_DRV_IO059"]
pub type Scudrvio059W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO059` reader - SCU_EN_SMT_IO059"]
pub type Scuensmtio059R = crate::BitReader;
#[doc = "Field `SCUENSMTIO059` writer - SCU_EN_SMT_IO059"]
pub type Scuensmtio059W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO059` reader - SCU_EN_HV_IO059"]
pub type Scuenhvio059R = crate::BitReader;
#[doc = "Field `SCUENHVIO059` writer - SCU_EN_HV_IO059"]
pub type Scuenhvio059W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO058"]
    #[inline(always)]
    pub fn scudispdio058(&self) -> Scudispdio058R {
        Scudispdio058R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO058"]
    #[inline(always)]
    pub fn scudispuio058(&self) -> Scudispuio058R {
        Scudispuio058R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO058"]
    #[inline(always)]
    pub fn scudrvio058(&self) -> Scudrvio058R {
        Scudrvio058R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO058"]
    #[inline(always)]
    pub fn scuensmtio058(&self) -> Scuensmtio058R {
        Scuensmtio058R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO058"]
    #[inline(always)]
    pub fn scuenhvio058(&self) -> Scuenhvio058R {
        Scuenhvio058R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO059"]
    #[inline(always)]
    pub fn scudispdio059(&self) -> Scudispdio059R {
        Scudispdio059R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO059"]
    #[inline(always)]
    pub fn scudispuio059(&self) -> Scudispuio059R {
        Scudispuio059R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO059"]
    #[inline(always)]
    pub fn scudrvio059(&self) -> Scudrvio059R {
        Scudrvio059R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO059"]
    #[inline(always)]
    pub fn scuensmtio059(&self) -> Scuensmtio059R {
        Scuensmtio059R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO059"]
    #[inline(always)]
    pub fn scuenhvio059(&self) -> Scuenhvio059R {
        Scuenhvio059R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO058"]
    #[inline(always)]
    pub fn scudispdio058(&mut self) -> Scudispdio058W<Scu4f4Spec> {
        Scudispdio058W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO058"]
    #[inline(always)]
    pub fn scudispuio058(&mut self) -> Scudispuio058W<Scu4f4Spec> {
        Scudispuio058W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO058"]
    #[inline(always)]
    pub fn scudrvio058(&mut self) -> Scudrvio058W<Scu4f4Spec> {
        Scudrvio058W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO058"]
    #[inline(always)]
    pub fn scuensmtio058(&mut self) -> Scuensmtio058W<Scu4f4Spec> {
        Scuensmtio058W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO058"]
    #[inline(always)]
    pub fn scuenhvio058(&mut self) -> Scuenhvio058W<Scu4f4Spec> {
        Scuenhvio058W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO059"]
    #[inline(always)]
    pub fn scudispdio059(&mut self) -> Scudispdio059W<Scu4f4Spec> {
        Scudispdio059W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO059"]
    #[inline(always)]
    pub fn scudispuio059(&mut self) -> Scudispuio059W<Scu4f4Spec> {
        Scudispuio059W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO059"]
    #[inline(always)]
    pub fn scudrvio059(&mut self) -> Scudrvio059W<Scu4f4Spec> {
        Scudrvio059W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO059"]
    #[inline(always)]
    pub fn scuensmtio059(&mut self) -> Scuensmtio059W<Scu4f4Spec> {
        Scuensmtio059W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO059"]
    #[inline(always)]
    pub fn scuenhvio059(&mut self) -> Scuenhvio059W<Scu4f4Spec> {
        Scuenhvio059W::new(self, 25)
    }
}
#[doc = "IO Control \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4f4Spec;
impl crate::RegisterSpec for Scu4f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4f4::R`](R) reader structure"]
impl crate::Readable for Scu4f4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4f4::W`](W) writer structure"]
impl crate::Writable for Scu4f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4F4 to value 0x0204_0204"]
impl crate::Resettable for Scu4f4Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
