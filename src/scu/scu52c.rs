#[doc = "Register `SCU52C` reader"]
pub type R = crate::R<Scu52cSpec>;
#[doc = "Register `SCU52C` writer"]
pub type W = crate::W<Scu52cSpec>;
#[doc = "Field `SCUDISPDIO086` reader - SCU_DIS_PD_IO086"]
pub type Scudispdio086R = crate::BitReader;
#[doc = "Field `SCUDISPDIO086` writer - SCU_DIS_PD_IO086"]
pub type Scudispdio086W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO086` reader - SCU_DIS_PU_IO086"]
pub type Scudispuio086R = crate::BitReader;
#[doc = "Field `SCUDISPUIO086` writer - SCU_DIS_PU_IO086"]
pub type Scudispuio086W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO086` reader - SCU_DRV_IO086"]
pub type Scudrvio086R = crate::FieldReader;
#[doc = "Field `SCUDRVIO086` writer - SCU_DRV_IO086"]
pub type Scudrvio086W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO086` reader - SCU_EN_SMT_IO086"]
pub type Scuensmtio086R = crate::BitReader;
#[doc = "Field `SCUENSMTIO086` writer - SCU_EN_SMT_IO086"]
pub type Scuensmtio086W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO086` reader - SCU_EN_HV_IO086"]
pub type Scuenhvio086R = crate::BitReader;
#[doc = "Field `SCUENHVIO086` writer - SCU_EN_HV_IO086"]
pub type Scuenhvio086W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO087` reader - SCU_DIS_PD_IO087"]
pub type Scudispdio087R = crate::BitReader;
#[doc = "Field `SCUDISPDIO087` writer - SCU_DIS_PD_IO087"]
pub type Scudispdio087W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO087` reader - SCU_DIS_PU_IO087"]
pub type Scudispuio087R = crate::BitReader;
#[doc = "Field `SCUDISPUIO087` writer - SCU_DIS_PU_IO087"]
pub type Scudispuio087W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO087` reader - SCU_DRV_IO087"]
pub type Scudrvio087R = crate::FieldReader;
#[doc = "Field `SCUDRVIO087` writer - SCU_DRV_IO087"]
pub type Scudrvio087W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO087` reader - SCU_EN_SMT_IO087"]
pub type Scuensmtio087R = crate::BitReader;
#[doc = "Field `SCUENSMTIO087` writer - SCU_EN_SMT_IO087"]
pub type Scuensmtio087W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO087` reader - SCU_EN_HV_IO087"]
pub type Scuenhvio087R = crate::BitReader;
#[doc = "Field `SCUENHVIO087` writer - SCU_EN_HV_IO087"]
pub type Scuenhvio087W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO086"]
    #[inline(always)]
    pub fn scudispdio086(&self) -> Scudispdio086R {
        Scudispdio086R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO086"]
    #[inline(always)]
    pub fn scudispuio086(&self) -> Scudispuio086R {
        Scudispuio086R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO086"]
    #[inline(always)]
    pub fn scudrvio086(&self) -> Scudrvio086R {
        Scudrvio086R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO086"]
    #[inline(always)]
    pub fn scuensmtio086(&self) -> Scuensmtio086R {
        Scuensmtio086R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO086"]
    #[inline(always)]
    pub fn scuenhvio086(&self) -> Scuenhvio086R {
        Scuenhvio086R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO087"]
    #[inline(always)]
    pub fn scudispdio087(&self) -> Scudispdio087R {
        Scudispdio087R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO087"]
    #[inline(always)]
    pub fn scudispuio087(&self) -> Scudispuio087R {
        Scudispuio087R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO087"]
    #[inline(always)]
    pub fn scudrvio087(&self) -> Scudrvio087R {
        Scudrvio087R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO087"]
    #[inline(always)]
    pub fn scuensmtio087(&self) -> Scuensmtio087R {
        Scuensmtio087R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO087"]
    #[inline(always)]
    pub fn scuenhvio087(&self) -> Scuenhvio087R {
        Scuenhvio087R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO086"]
    #[inline(always)]
    pub fn scudispdio086(&mut self) -> Scudispdio086W<Scu52cSpec> {
        Scudispdio086W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO086"]
    #[inline(always)]
    pub fn scudispuio086(&mut self) -> Scudispuio086W<Scu52cSpec> {
        Scudispuio086W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO086"]
    #[inline(always)]
    pub fn scudrvio086(&mut self) -> Scudrvio086W<Scu52cSpec> {
        Scudrvio086W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO086"]
    #[inline(always)]
    pub fn scuensmtio086(&mut self) -> Scuensmtio086W<Scu52cSpec> {
        Scuensmtio086W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO086"]
    #[inline(always)]
    pub fn scuenhvio086(&mut self) -> Scuenhvio086W<Scu52cSpec> {
        Scuenhvio086W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO087"]
    #[inline(always)]
    pub fn scudispdio087(&mut self) -> Scudispdio087W<Scu52cSpec> {
        Scudispdio087W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO087"]
    #[inline(always)]
    pub fn scudispuio087(&mut self) -> Scudispuio087W<Scu52cSpec> {
        Scudispuio087W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO087"]
    #[inline(always)]
    pub fn scudrvio087(&mut self) -> Scudrvio087W<Scu52cSpec> {
        Scudrvio087W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO087"]
    #[inline(always)]
    pub fn scuensmtio087(&mut self) -> Scuensmtio087W<Scu52cSpec> {
        Scuensmtio087W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO087"]
    #[inline(always)]
    pub fn scuenhvio087(&mut self) -> Scuenhvio087W<Scu52cSpec> {
        Scuenhvio087W::new(self, 25)
    }
}
#[doc = "IO Control \\#44\n\nYou can [`read`](crate::Reg::read) this register and get [`scu52c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu52c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu52cSpec;
impl crate::RegisterSpec for Scu52cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu52c::R`](R) reader structure"]
impl crate::Readable for Scu52cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu52c::W`](W) writer structure"]
impl crate::Writable for Scu52cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU52C to value 0x0204_0104"]
impl crate::Resettable for Scu52cSpec {
    const RESET_VALUE: u32 = 0x0204_0104;
}
