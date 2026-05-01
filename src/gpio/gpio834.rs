#[doc = "Register `GPIO834` reader"]
pub type R = crate::R<Gpio834Spec>;
#[doc = "Register `GPIO834` writer"]
pub type W = crate::W<Gpio834Spec>;
#[doc = "Field `GPIO036WrPrivilegeOfMaster` reader - GPIO036 Write Privilege of Master"]
pub type Gpio036wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO036WrPrivilegeOfMaster` writer - GPIO036 Write Privilege of Master"]
pub type Gpio036wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO037WrPrivilegeOfMaster` reader - GPIO037 Write Privilege of Master"]
pub type Gpio037wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO037WrPrivilegeOfMaster` writer - GPIO037 Write Privilege of Master"]
pub type Gpio037wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO038WrPrivilegeOfMaster` reader - GPIO038 Write Privilege of Master"]
pub type Gpio038wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO038WrPrivilegeOfMaster` writer - GPIO038 Write Privilege of Master"]
pub type Gpio038wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO039WrPrivilegeOfMaster` reader - GPIO039 Write Privilege of Master"]
pub type Gpio039wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO039WrPrivilegeOfMaster` writer - GPIO039 Write Privilege of Master"]
pub type Gpio039wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO036 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio036wr_privilege_of_master(&self) -> Gpio036wrPrivilegeOfMasterR {
        Gpio036wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO037 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio037wr_privilege_of_master(&self) -> Gpio037wrPrivilegeOfMasterR {
        Gpio037wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO038 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio038wr_privilege_of_master(&self) -> Gpio038wrPrivilegeOfMasterR {
        Gpio038wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO039 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio039wr_privilege_of_master(&self) -> Gpio039wrPrivilegeOfMasterR {
        Gpio039wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO036 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio036wr_privilege_of_master(&mut self) -> Gpio036wrPrivilegeOfMasterW<Gpio834Spec> {
        Gpio036wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO037 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio037wr_privilege_of_master(&mut self) -> Gpio037wrPrivilegeOfMasterW<Gpio834Spec> {
        Gpio037wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO038 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio038wr_privilege_of_master(&mut self) -> Gpio038wrPrivilegeOfMasterW<Gpio834Spec> {
        Gpio038wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO039 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio039wr_privilege_of_master(&mut self) -> Gpio039wrPrivilegeOfMasterW<Gpio834Spec> {
        Gpio039wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio834::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio834::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio834Spec;
impl crate::RegisterSpec for Gpio834Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio834::R`](R) reader structure"]
impl crate::Readable for Gpio834Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio834::W`](W) writer structure"]
impl crate::Writable for Gpio834Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO834 to value 0xffff_ffff"]
impl crate::Resettable for Gpio834Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
