#[doc = "Register `SCU56C` reader"]
pub type R = crate::R<Scu56cSpec>;
#[doc = "Register `SCU56C` writer"]
pub type W = crate::W<Scu56cSpec>;
#[doc = "Field `SCUDISPDIO118` reader - SCU_DIS_PD_IO118"]
pub type Scudispdio118R = crate::BitReader;
#[doc = "Field `SCUDISPDIO118` writer - SCU_DIS_PD_IO118"]
pub type Scudispdio118W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO118` reader - SCU_DIS_PU_IO118"]
pub type Scudispuio118R = crate::BitReader;
#[doc = "Field `SCUDISPUIO118` writer - SCU_DIS_PU_IO118"]
pub type Scudispuio118W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO118` reader - SCU_DRV_IO118"]
pub type Scudrvio118R = crate::FieldReader;
#[doc = "Field `SCUDRVIO118` writer - SCU_DRV_IO118"]
pub type Scudrvio118W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO118` reader - SCU_EN_SMT_IO118"]
pub type Scuensmtio118R = crate::BitReader;
#[doc = "Field `SCUENSMTIO118` writer - SCU_EN_SMT_IO118"]
pub type Scuensmtio118W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO118` reader - SCU_EN_HV_IO118"]
pub type Scuenhvio118R = crate::BitReader;
#[doc = "Field `SCUENHVIO118` writer - SCU_EN_HV_IO118"]
pub type Scuenhvio118W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO119` reader - SCU_DIS_PD_IO119"]
pub type Scudispdio119R = crate::BitReader;
#[doc = "Field `SCUDISPDIO119` writer - SCU_DIS_PD_IO119"]
pub type Scudispdio119W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO119` reader - SCU_DIS_PU_IO119"]
pub type Scudispuio119R = crate::BitReader;
#[doc = "Field `SCUDISPUIO119` writer - SCU_DIS_PU_IO119"]
pub type Scudispuio119W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO119` reader - SCU_DRV_IO119"]
pub type Scudrvio119R = crate::FieldReader;
#[doc = "Field `SCUDRVIO119` writer - SCU_DRV_IO119"]
pub type Scudrvio119W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO119` reader - SCU_EN_SMT_IO119"]
pub type Scuensmtio119R = crate::BitReader;
#[doc = "Field `SCUENSMTIO119` writer - SCU_EN_SMT_IO119"]
pub type Scuensmtio119W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO119` reader - SCU_EN_HV_IO119"]
pub type Scuenhvio119R = crate::BitReader;
#[doc = "Field `SCUENHVIO119` writer - SCU_EN_HV_IO119"]
pub type Scuenhvio119W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO118"]
    #[inline(always)]
    pub fn scudispdio118(&self) -> Scudispdio118R {
        Scudispdio118R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO118"]
    #[inline(always)]
    pub fn scudispuio118(&self) -> Scudispuio118R {
        Scudispuio118R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO118"]
    #[inline(always)]
    pub fn scudrvio118(&self) -> Scudrvio118R {
        Scudrvio118R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO118"]
    #[inline(always)]
    pub fn scuensmtio118(&self) -> Scuensmtio118R {
        Scuensmtio118R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO118"]
    #[inline(always)]
    pub fn scuenhvio118(&self) -> Scuenhvio118R {
        Scuenhvio118R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO119"]
    #[inline(always)]
    pub fn scudispdio119(&self) -> Scudispdio119R {
        Scudispdio119R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO119"]
    #[inline(always)]
    pub fn scudispuio119(&self) -> Scudispuio119R {
        Scudispuio119R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO119"]
    #[inline(always)]
    pub fn scudrvio119(&self) -> Scudrvio119R {
        Scudrvio119R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO119"]
    #[inline(always)]
    pub fn scuensmtio119(&self) -> Scuensmtio119R {
        Scuensmtio119R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO119"]
    #[inline(always)]
    pub fn scuenhvio119(&self) -> Scuenhvio119R {
        Scuenhvio119R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO118"]
    #[inline(always)]
    pub fn scudispdio118(&mut self) -> Scudispdio118W<Scu56cSpec> {
        Scudispdio118W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO118"]
    #[inline(always)]
    pub fn scudispuio118(&mut self) -> Scudispuio118W<Scu56cSpec> {
        Scudispuio118W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO118"]
    #[inline(always)]
    pub fn scudrvio118(&mut self) -> Scudrvio118W<Scu56cSpec> {
        Scudrvio118W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO118"]
    #[inline(always)]
    pub fn scuensmtio118(&mut self) -> Scuensmtio118W<Scu56cSpec> {
        Scuensmtio118W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO118"]
    #[inline(always)]
    pub fn scuenhvio118(&mut self) -> Scuenhvio118W<Scu56cSpec> {
        Scuenhvio118W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO119"]
    #[inline(always)]
    pub fn scudispdio119(&mut self) -> Scudispdio119W<Scu56cSpec> {
        Scudispdio119W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO119"]
    #[inline(always)]
    pub fn scudispuio119(&mut self) -> Scudispuio119W<Scu56cSpec> {
        Scudispuio119W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO119"]
    #[inline(always)]
    pub fn scudrvio119(&mut self) -> Scudrvio119W<Scu56cSpec> {
        Scudrvio119W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO119"]
    #[inline(always)]
    pub fn scuensmtio119(&mut self) -> Scuensmtio119W<Scu56cSpec> {
        Scuensmtio119W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO119"]
    #[inline(always)]
    pub fn scuenhvio119(&mut self) -> Scuenhvio119W<Scu56cSpec> {
        Scuenhvio119W::new(self, 25)
    }
}
#[doc = "IO Control \\#60\n\nYou can [`read`](crate::Reg::read) this register and get [`scu56c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu56c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu56cSpec;
impl crate::RegisterSpec for Scu56cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu56c::R`](R) reader structure"]
impl crate::Readable for Scu56cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu56c::W`](W) writer structure"]
impl crate::Writable for Scu56cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU56C to value 0x0204_0204"]
impl crate::Resettable for Scu56cSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
