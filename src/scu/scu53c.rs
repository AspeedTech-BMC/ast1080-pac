#[doc = "Register `SCU53C` reader"]
pub type R = crate::R<Scu53cSpec>;
#[doc = "Register `SCU53C` writer"]
pub type W = crate::W<Scu53cSpec>;
#[doc = "Field `SCUDISPDIO094` reader - SCU_DIS_PD_IO094"]
pub type Scudispdio094R = crate::BitReader;
#[doc = "Field `SCUDISPDIO094` writer - SCU_DIS_PD_IO094"]
pub type Scudispdio094W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO094` reader - SCU_DIS_PU_IO094"]
pub type Scudispuio094R = crate::BitReader;
#[doc = "Field `SCUDISPUIO094` writer - SCU_DIS_PU_IO094"]
pub type Scudispuio094W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO094` reader - SCU_DRV_IO094"]
pub type Scudrvio094R = crate::FieldReader;
#[doc = "Field `SCUDRVIO094` writer - SCU_DRV_IO094"]
pub type Scudrvio094W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO094` reader - SCU_EN_SMT_IO094"]
pub type Scuensmtio094R = crate::BitReader;
#[doc = "Field `SCUENSMTIO094` writer - SCU_EN_SMT_IO094"]
pub type Scuensmtio094W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO094` reader - SCU_EN_HV_IO094"]
pub type Scuenhvio094R = crate::BitReader;
#[doc = "Field `SCUENHVIO094` writer - SCU_EN_HV_IO094"]
pub type Scuenhvio094W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO095` reader - SCU_DIS_PD_IO095"]
pub type Scudispdio095R = crate::BitReader;
#[doc = "Field `SCUDISPDIO095` writer - SCU_DIS_PD_IO095"]
pub type Scudispdio095W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO095` reader - SCU_DIS_PU_IO095"]
pub type Scudispuio095R = crate::BitReader;
#[doc = "Field `SCUDISPUIO095` writer - SCU_DIS_PU_IO095"]
pub type Scudispuio095W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO095` reader - SCU_DRV_IO095"]
pub type Scudrvio095R = crate::FieldReader;
#[doc = "Field `SCUDRVIO095` writer - SCU_DRV_IO095"]
pub type Scudrvio095W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO095` reader - SCU_EN_SMT_IO095"]
pub type Scuensmtio095R = crate::BitReader;
#[doc = "Field `SCUENSMTIO095` writer - SCU_EN_SMT_IO095"]
pub type Scuensmtio095W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO095` reader - SCU_EN_HV_IO095"]
pub type Scuenhvio095R = crate::BitReader;
#[doc = "Field `SCUENHVIO095` writer - SCU_EN_HV_IO095"]
pub type Scuenhvio095W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO094"]
    #[inline(always)]
    pub fn scudispdio094(&self) -> Scudispdio094R {
        Scudispdio094R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO094"]
    #[inline(always)]
    pub fn scudispuio094(&self) -> Scudispuio094R {
        Scudispuio094R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO094"]
    #[inline(always)]
    pub fn scudrvio094(&self) -> Scudrvio094R {
        Scudrvio094R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO094"]
    #[inline(always)]
    pub fn scuensmtio094(&self) -> Scuensmtio094R {
        Scuensmtio094R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO094"]
    #[inline(always)]
    pub fn scuenhvio094(&self) -> Scuenhvio094R {
        Scuenhvio094R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO095"]
    #[inline(always)]
    pub fn scudispdio095(&self) -> Scudispdio095R {
        Scudispdio095R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO095"]
    #[inline(always)]
    pub fn scudispuio095(&self) -> Scudispuio095R {
        Scudispuio095R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO095"]
    #[inline(always)]
    pub fn scudrvio095(&self) -> Scudrvio095R {
        Scudrvio095R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO095"]
    #[inline(always)]
    pub fn scuensmtio095(&self) -> Scuensmtio095R {
        Scuensmtio095R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO095"]
    #[inline(always)]
    pub fn scuenhvio095(&self) -> Scuenhvio095R {
        Scuenhvio095R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO094"]
    #[inline(always)]
    pub fn scudispdio094(&mut self) -> Scudispdio094W<Scu53cSpec> {
        Scudispdio094W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO094"]
    #[inline(always)]
    pub fn scudispuio094(&mut self) -> Scudispuio094W<Scu53cSpec> {
        Scudispuio094W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO094"]
    #[inline(always)]
    pub fn scudrvio094(&mut self) -> Scudrvio094W<Scu53cSpec> {
        Scudrvio094W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO094"]
    #[inline(always)]
    pub fn scuensmtio094(&mut self) -> Scuensmtio094W<Scu53cSpec> {
        Scuensmtio094W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO094"]
    #[inline(always)]
    pub fn scuenhvio094(&mut self) -> Scuenhvio094W<Scu53cSpec> {
        Scuenhvio094W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO095"]
    #[inline(always)]
    pub fn scudispdio095(&mut self) -> Scudispdio095W<Scu53cSpec> {
        Scudispdio095W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO095"]
    #[inline(always)]
    pub fn scudispuio095(&mut self) -> Scudispuio095W<Scu53cSpec> {
        Scudispuio095W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO095"]
    #[inline(always)]
    pub fn scudrvio095(&mut self) -> Scudrvio095W<Scu53cSpec> {
        Scudrvio095W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO095"]
    #[inline(always)]
    pub fn scuensmtio095(&mut self) -> Scuensmtio095W<Scu53cSpec> {
        Scuensmtio095W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO095"]
    #[inline(always)]
    pub fn scuenhvio095(&mut self) -> Scuenhvio095W<Scu53cSpec> {
        Scuenhvio095W::new(self, 25)
    }
}
#[doc = "IO Control \\#48\n\nYou can [`read`](crate::Reg::read) this register and get [`scu53c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu53c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu53cSpec;
impl crate::RegisterSpec for Scu53cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu53c::R`](R) reader structure"]
impl crate::Readable for Scu53cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu53c::W`](W) writer structure"]
impl crate::Writable for Scu53cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU53C to value 0x0204_0204"]
impl crate::Resettable for Scu53cSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
