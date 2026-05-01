#[doc = "Register `SCU680` reader"]
pub type R = crate::R<Scu680Spec>;
#[doc = "Register `SCU680` writer"]
pub type W = crate::W<Scu680Spec>;
#[doc = "Field `SCUIOCTRLGPD` reader - SCU_IO_CTRL_G_PD"]
pub type ScuioctrlgpdR = crate::FieldReader;
#[doc = "Field `SCUIOCTRLGPD` writer - SCU_IO_CTRL_G_PD"]
pub type ScuioctrlgpdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLGPU` reader - SCU_IO_CTRL_G_PU"]
pub type ScuioctrlgpuR = crate::FieldReader;
#[doc = "Field `SCUIOCTRLGPU` writer - SCU_IO_CTRL_G_PU"]
pub type ScuioctrlgpuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLGPDIO` reader - SCU_IO_CTRL_G_PDIO"]
pub type ScuioctrlgpdioR = crate::FieldReader;
#[doc = "Field `SCUIOCTRLGPDIO` writer - SCU_IO_CTRL_G_PDIO"]
pub type ScuioctrlgpdioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLGPUIO` reader - SCU_IO_CTRL_G_PUIO"]
pub type ScuioctrlgpuioR = crate::FieldReader;
#[doc = "Field `SCUIOCTRLGPUIO` writer - SCU_IO_CTRL_G_PUIO"]
pub type ScuioctrlgpuioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLGNCAL` reader - SCU_IO_CTRL_G_NCAL"]
pub type ScuioctrlgncalR = crate::BitReader;
#[doc = "Field `SCUIOCTRLGNCAL` writer - SCU_IO_CTRL_G_NCAL"]
pub type ScuioctrlgncalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUIOCTRLGPCAL` reader - SCU_IO_CTRL_G_PCAL"]
pub type ScuioctrlgpcalR = crate::BitReader;
#[doc = "Field `SCUIOCTRLGPCAL` writer - SCU_IO_CTRL_G_PCAL"]
pub type ScuioctrlgpcalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLGA2` reader - SCU_IO_CTRL_G_A2"]
pub type Scuioctrlga2R = crate::BitReader;
#[doc = "Field `SCUIOCTRLGA2` writer - SCU_IO_CTRL_G_A2"]
pub type Scuioctrlga2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUIOCTRLGA6` reader - SCU_IO_CTRL_G_A6"]
pub type Scuioctrlga6R = crate::BitReader;
#[doc = "Field `SCUIOCTRLGA6` writer - SCU_IO_CTRL_G_A6"]
pub type Scuioctrlga6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUIOCTRLGA9` reader - SCU_IO_CTRL_G_A9"]
pub type Scuioctrlga9R = crate::BitReader;
#[doc = "Field `SCUIOCTRLGA9` writer - SCU_IO_CTRL_G_A9"]
pub type Scuioctrlga9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUIOCTRLGDS` reader - SCU_IO_CTRL_G_DS"]
pub type ScuioctrlgdsR = crate::FieldReader;
#[doc = "Field `SCUIOCTRLGDS` writer - SCU_IO_CTRL_G_DS"]
pub type ScuioctrlgdsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - SCU_IO_CTRL_G_PD"]
    #[inline(always)]
    pub fn scuioctrlgpd(&self) -> ScuioctrlgpdR {
        ScuioctrlgpdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SCU_IO_CTRL_G_PU"]
    #[inline(always)]
    pub fn scuioctrlgpu(&self) -> ScuioctrlgpuR {
        ScuioctrlgpuR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SCU_IO_CTRL_G_PDIO"]
    #[inline(always)]
    pub fn scuioctrlgpdio(&self) -> ScuioctrlgpdioR {
        ScuioctrlgpdioR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SCU_IO_CTRL_G_PUIO"]
    #[inline(always)]
    pub fn scuioctrlgpuio(&self) -> ScuioctrlgpuioR {
        ScuioctrlgpuioR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - SCU_IO_CTRL_G_NCAL"]
    #[inline(always)]
    pub fn scuioctrlgncal(&self) -> ScuioctrlgncalR {
        ScuioctrlgncalR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_IO_CTRL_G_PCAL"]
    #[inline(always)]
    pub fn scuioctrlgpcal(&self) -> ScuioctrlgpcalR {
        ScuioctrlgpcalR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - SCU_IO_CTRL_G_A2"]
    #[inline(always)]
    pub fn scuioctrlga2(&self) -> Scuioctrlga2R {
        Scuioctrlga2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_IO_CTRL_G_A6"]
    #[inline(always)]
    pub fn scuioctrlga6(&self) -> Scuioctrlga6R {
        Scuioctrlga6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SCU_IO_CTRL_G_A9"]
    #[inline(always)]
    pub fn scuioctrlga9(&self) -> Scuioctrlga9R {
        Scuioctrlga9R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - SCU_IO_CTRL_G_DS"]
    #[inline(always)]
    pub fn scuioctrlgds(&self) -> ScuioctrlgdsR {
        ScuioctrlgdsR::new(((self.bits >> 23) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SCU_IO_CTRL_G_PD"]
    #[inline(always)]
    pub fn scuioctrlgpd(&mut self) -> ScuioctrlgpdW<Scu680Spec> {
        ScuioctrlgpdW::new(self, 0)
    }
    #[doc = "Bits 4:7 - SCU_IO_CTRL_G_PU"]
    #[inline(always)]
    pub fn scuioctrlgpu(&mut self) -> ScuioctrlgpuW<Scu680Spec> {
        ScuioctrlgpuW::new(self, 4)
    }
    #[doc = "Bits 8:11 - SCU_IO_CTRL_G_PDIO"]
    #[inline(always)]
    pub fn scuioctrlgpdio(&mut self) -> ScuioctrlgpdioW<Scu680Spec> {
        ScuioctrlgpdioW::new(self, 8)
    }
    #[doc = "Bits 12:15 - SCU_IO_CTRL_G_PUIO"]
    #[inline(always)]
    pub fn scuioctrlgpuio(&mut self) -> ScuioctrlgpuioW<Scu680Spec> {
        ScuioctrlgpuioW::new(self, 12)
    }
    #[doc = "Bit 16 - SCU_IO_CTRL_G_NCAL"]
    #[inline(always)]
    pub fn scuioctrlgncal(&mut self) -> ScuioctrlgncalW<Scu680Spec> {
        ScuioctrlgncalW::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_IO_CTRL_G_PCAL"]
    #[inline(always)]
    pub fn scuioctrlgpcal(&mut self) -> ScuioctrlgpcalW<Scu680Spec> {
        ScuioctrlgpcalW::new(self, 17)
    }
    #[doc = "Bit 20 - SCU_IO_CTRL_G_A2"]
    #[inline(always)]
    pub fn scuioctrlga2(&mut self) -> Scuioctrlga2W<Scu680Spec> {
        Scuioctrlga2W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_IO_CTRL_G_A6"]
    #[inline(always)]
    pub fn scuioctrlga6(&mut self) -> Scuioctrlga6W<Scu680Spec> {
        Scuioctrlga6W::new(self, 21)
    }
    #[doc = "Bit 22 - SCU_IO_CTRL_G_A9"]
    #[inline(always)]
    pub fn scuioctrlga9(&mut self) -> Scuioctrlga9W<Scu680Spec> {
        Scuioctrlga9W::new(self, 22)
    }
    #[doc = "Bits 23:24 - SCU_IO_CTRL_G_DS"]
    #[inline(always)]
    pub fn scuioctrlgds(&mut self) -> ScuioctrlgdsW<Scu680Spec> {
        ScuioctrlgdsW::new(self, 23)
    }
}
#[doc = "IO Control 0 Regiser\n\nYou can [`read`](crate::Reg::read) this register and get [`scu680::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu680::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu680Spec;
impl crate::RegisterSpec for Scu680Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu680::R`](R) reader structure"]
impl crate::Readable for Scu680Spec {}
#[doc = "`write(|w| ..)` method takes [`scu680::W`](W) writer structure"]
impl crate::Writable for Scu680Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU680 to value 0x4a0f"]
impl crate::Resettable for Scu680Spec {
    const RESET_VALUE: u32 = 0x4a0f;
}
