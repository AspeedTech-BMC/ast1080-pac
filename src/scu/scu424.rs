#[doc = "Register `SCU424` reader"]
pub type R = crate::R<Scu424Spec>;
#[doc = "Register `SCU424` writer"]
pub type W = crate::W<Scu424Spec>;
#[doc = "Field `SCUMUXIO072` reader - SCU_MUX_IO072"]
pub type Scumuxio072R = crate::FieldReader;
#[doc = "Field `SCUMUXIO072` writer - SCU_MUX_IO072"]
pub type Scumuxio072W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO073` reader - SCU_MUX_IO073"]
pub type Scumuxio073R = crate::FieldReader;
#[doc = "Field `SCUMUXIO073` writer - SCU_MUX_IO073"]
pub type Scumuxio073W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO074` reader - SCU_MUX_IO074"]
pub type Scumuxio074R = crate::FieldReader;
#[doc = "Field `SCUMUXIO074` writer - SCU_MUX_IO074"]
pub type Scumuxio074W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO075` reader - SCU_MUX_IO075"]
pub type Scumuxio075R = crate::FieldReader;
#[doc = "Field `SCUMUXIO075` writer - SCU_MUX_IO075"]
pub type Scumuxio075W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO076` reader - SCU_MUX_IO076"]
pub type Scumuxio076R = crate::FieldReader;
#[doc = "Field `SCUMUXIO076` writer - SCU_MUX_IO076"]
pub type Scumuxio076W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO077` reader - SCU_MUX_IO077"]
pub type Scumuxio077R = crate::FieldReader;
#[doc = "Field `SCUMUXIO077` writer - SCU_MUX_IO077"]
pub type Scumuxio077W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO078` reader - SCU_MUX_IO078"]
pub type Scumuxio078R = crate::FieldReader;
#[doc = "Field `SCUMUXIO078` writer - SCU_MUX_IO078"]
pub type Scumuxio078W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO079` reader - SCU_MUX_IO079"]
pub type Scumuxio079R = crate::FieldReader;
#[doc = "Field `SCUMUXIO079` writer - SCU_MUX_IO079"]
pub type Scumuxio079W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO072"]
    #[inline(always)]
    pub fn scumuxio072(&self) -> Scumuxio072R {
        Scumuxio072R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO073"]
    #[inline(always)]
    pub fn scumuxio073(&self) -> Scumuxio073R {
        Scumuxio073R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO074"]
    #[inline(always)]
    pub fn scumuxio074(&self) -> Scumuxio074R {
        Scumuxio074R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO075"]
    #[inline(always)]
    pub fn scumuxio075(&self) -> Scumuxio075R {
        Scumuxio075R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO076"]
    #[inline(always)]
    pub fn scumuxio076(&self) -> Scumuxio076R {
        Scumuxio076R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO077"]
    #[inline(always)]
    pub fn scumuxio077(&self) -> Scumuxio077R {
        Scumuxio077R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO078"]
    #[inline(always)]
    pub fn scumuxio078(&self) -> Scumuxio078R {
        Scumuxio078R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO079"]
    #[inline(always)]
    pub fn scumuxio079(&self) -> Scumuxio079R {
        Scumuxio079R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO072"]
    #[inline(always)]
    pub fn scumuxio072(&mut self) -> Scumuxio072W<Scu424Spec> {
        Scumuxio072W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO073"]
    #[inline(always)]
    pub fn scumuxio073(&mut self) -> Scumuxio073W<Scu424Spec> {
        Scumuxio073W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO074"]
    #[inline(always)]
    pub fn scumuxio074(&mut self) -> Scumuxio074W<Scu424Spec> {
        Scumuxio074W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO075"]
    #[inline(always)]
    pub fn scumuxio075(&mut self) -> Scumuxio075W<Scu424Spec> {
        Scumuxio075W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO076"]
    #[inline(always)]
    pub fn scumuxio076(&mut self) -> Scumuxio076W<Scu424Spec> {
        Scumuxio076W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO077"]
    #[inline(always)]
    pub fn scumuxio077(&mut self) -> Scumuxio077W<Scu424Spec> {
        Scumuxio077W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO078"]
    #[inline(always)]
    pub fn scumuxio078(&mut self) -> Scumuxio078W<Scu424Spec> {
        Scumuxio078W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO079"]
    #[inline(always)]
    pub fn scumuxio079(&mut self) -> Scumuxio079W<Scu424Spec> {
        Scumuxio079W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`scu424::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu424::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu424Spec;
impl crate::RegisterSpec for Scu424Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu424::R`](R) reader structure"]
impl crate::Readable for Scu424Spec {}
#[doc = "`write(|w| ..)` method takes [`scu424::W`](W) writer structure"]
impl crate::Writable for Scu424Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU424 to value 0"]
impl crate::Resettable for Scu424Spec {}
