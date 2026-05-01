#[doc = "Register `SCU4A8` reader"]
pub type R = crate::R<Scu4a8Spec>;
#[doc = "Register `SCU4A8` writer"]
pub type W = crate::W<Scu4a8Spec>;
#[doc = "Field `SCUDISPDIO020` reader - SCU_DIS_PD_IO020"]
pub type Scudispdio020R = crate::BitReader;
#[doc = "Field `SCUDISPDIO020` writer - SCU_DIS_PD_IO020"]
pub type Scudispdio020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO020` reader - SCU_DIS_PU_IO020"]
pub type Scudispuio020R = crate::BitReader;
#[doc = "Field `SCUDISPUIO020` writer - SCU_DIS_PU_IO020"]
pub type Scudispuio020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO020` reader - SCU_DRV_IO020"]
pub type Scudrvio020R = crate::FieldReader;
#[doc = "Field `SCUDRVIO020` writer - SCU_DRV_IO020"]
pub type Scudrvio020W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO020` reader - SCU_EN_SMT_IO020"]
pub type Scuensmtio020R = crate::BitReader;
#[doc = "Field `SCUENSMTIO020` writer - SCU_EN_SMT_IO020"]
pub type Scuensmtio020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO020` reader - SCU_EN_HV_IO020"]
pub type Scuenhvio020R = crate::BitReader;
#[doc = "Field `SCUENHVIO020` writer - SCU_EN_HV_IO020"]
pub type Scuenhvio020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO021` reader - SCU_DIS_PD_IO021"]
pub type Scudispdio021R = crate::BitReader;
#[doc = "Field `SCUDISPDIO021` writer - SCU_DIS_PD_IO021"]
pub type Scudispdio021W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO021` reader - SCU_DIS_PU_IO021"]
pub type Scudispuio021R = crate::BitReader;
#[doc = "Field `SCUDISPUIO021` writer - SCU_DIS_PU_IO021"]
pub type Scudispuio021W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO021` reader - SCU_DRV_IO021"]
pub type Scudrvio021R = crate::FieldReader;
#[doc = "Field `SCUDRVIO021` writer - SCU_DRV_IO021"]
pub type Scudrvio021W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO021` reader - SCU_EN_SMT_IO021"]
pub type Scuensmtio021R = crate::BitReader;
#[doc = "Field `SCUENSMTIO021` writer - SCU_EN_SMT_IO021"]
pub type Scuensmtio021W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO021` reader - SCU_EN_HV_IO021"]
pub type Scuenhvio021R = crate::BitReader;
#[doc = "Field `SCUENHVIO021` writer - SCU_EN_HV_IO021"]
pub type Scuenhvio021W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO020"]
    #[inline(always)]
    pub fn scudispdio020(&self) -> Scudispdio020R {
        Scudispdio020R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO020"]
    #[inline(always)]
    pub fn scudispuio020(&self) -> Scudispuio020R {
        Scudispuio020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO020"]
    #[inline(always)]
    pub fn scudrvio020(&self) -> Scudrvio020R {
        Scudrvio020R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO020"]
    #[inline(always)]
    pub fn scuensmtio020(&self) -> Scuensmtio020R {
        Scuensmtio020R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO020"]
    #[inline(always)]
    pub fn scuenhvio020(&self) -> Scuenhvio020R {
        Scuenhvio020R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO021"]
    #[inline(always)]
    pub fn scudispdio021(&self) -> Scudispdio021R {
        Scudispdio021R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO021"]
    #[inline(always)]
    pub fn scudispuio021(&self) -> Scudispuio021R {
        Scudispuio021R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO021"]
    #[inline(always)]
    pub fn scudrvio021(&self) -> Scudrvio021R {
        Scudrvio021R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO021"]
    #[inline(always)]
    pub fn scuensmtio021(&self) -> Scuensmtio021R {
        Scuensmtio021R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO021"]
    #[inline(always)]
    pub fn scuenhvio021(&self) -> Scuenhvio021R {
        Scuenhvio021R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO020"]
    #[inline(always)]
    pub fn scudispdio020(&mut self) -> Scudispdio020W<Scu4a8Spec> {
        Scudispdio020W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO020"]
    #[inline(always)]
    pub fn scudispuio020(&mut self) -> Scudispuio020W<Scu4a8Spec> {
        Scudispuio020W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO020"]
    #[inline(always)]
    pub fn scudrvio020(&mut self) -> Scudrvio020W<Scu4a8Spec> {
        Scudrvio020W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO020"]
    #[inline(always)]
    pub fn scuensmtio020(&mut self) -> Scuensmtio020W<Scu4a8Spec> {
        Scuensmtio020W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO020"]
    #[inline(always)]
    pub fn scuenhvio020(&mut self) -> Scuenhvio020W<Scu4a8Spec> {
        Scuenhvio020W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO021"]
    #[inline(always)]
    pub fn scudispdio021(&mut self) -> Scudispdio021W<Scu4a8Spec> {
        Scudispdio021W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO021"]
    #[inline(always)]
    pub fn scudispuio021(&mut self) -> Scudispuio021W<Scu4a8Spec> {
        Scudispuio021W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO021"]
    #[inline(always)]
    pub fn scudrvio021(&mut self) -> Scudrvio021W<Scu4a8Spec> {
        Scudrvio021W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO021"]
    #[inline(always)]
    pub fn scuensmtio021(&mut self) -> Scuensmtio021W<Scu4a8Spec> {
        Scuensmtio021W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO021"]
    #[inline(always)]
    pub fn scuenhvio021(&mut self) -> Scuenhvio021W<Scu4a8Spec> {
        Scuenhvio021W::new(self, 25)
    }
}
#[doc = "IO Control \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4a8Spec;
impl crate::RegisterSpec for Scu4a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4a8::R`](R) reader structure"]
impl crate::Readable for Scu4a8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4a8::W`](W) writer structure"]
impl crate::Writable for Scu4a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4A8 to value 0x0204_0204"]
impl crate::Resettable for Scu4a8Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
